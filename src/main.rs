use arcalog::{collection::prow::*, system::check_slash};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_with::*;
use serde_yaml;
use std::fs::File;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    collection: Option<Collection>,
    data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Collection {
    prow: Option<Prow>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Prow {
    location: Vec<String>,
    parameters: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildId {
    build_id: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildComparisons {
    list_of_builds: String,
}

#[derive(Parser, Debug)]
#[clap(
    name = "   __    ____   ___    __    __    _____  ___ 
  /__\\  (  _ \\ / __)  /__\\  (  )  (  _  )/ __)
 /(__)\\  )   /( (__  /(__)\\  )(__  )(_)(( (_-.
(__)(__)(_)\\_) \\___)(__)(__)(____)(_____)\\___/",
    author = "Arcalot Community",
    version = "\n0.0.1",
    about,
    long_about = "Join us at the Arcalot Round Table!"
)]
struct Args {
    /// Path of the config file
    #[clap(long, value_parser, default_value = "config.yaml")]
    config: String,
    /// Desired path to where the data should be collected, defaults to the path specified in your config file (or the current directory, if absent from config file)
    #[clap(long, value_parser, default_value = "data/")]
    data: String,
    /// Comma-separated list of parameters to pass collection sources mentioned in the configuration file (do not use spaces)
    #[clap(long, value_parser, default_value = "")]
    collect: String,
    /// If this flag is present, collects all artifacts in all relevant folders from the collection sources
    #[clap(short = 'a', long, value_parser)]
    artifacts: bool,
    /// Starts the Arcalog web server
    #[clap(long, value_parser)]
    http: bool,
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../static/404.html")),
    )
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn handler_api_build(build_info: Query<BuildId>, source_path: String) -> Json<BuildInfo> {
    let build_info = get_build_info(build_info.build_id.to_string(), source_path).await;
    return Json(build_info);
}

async fn handler_api_compare(build_info: Query<BuildComparisons>) -> Html<String> {
    let comparison_list = build_info.list_of_builds.split(",");
    Html(format!(
        "Going to implement comparison of analysis results between several build IDs. You queried these IDs: {}!",
        comparison_list.collect::<Vec<&str>>().join(", ")
    ))
}

async fn handler_build_id() -> impl IntoResponse {
    (
        StatusCode::OK,
        "This is for build ID responses with the struct Job",
    )
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let artifacts_collection = args.artifacts;
    let http_server = args.http;
    let config_location = args.config;
    let config = File::open(config_location).expect("Could not open config file");
    let config: Root = serde_yaml::from_reader(config).expect("Could not parse config file");
    let data_path_from_args = args.data;
    let collect = args.collect;
    let data_path_from_cfg = config.data.unwrap_or("".to_string());
    let mut data_path = String::from("data/");
    if data_path_from_args != "data/" {
        data_path = check_slash(&data_path_from_args);
    } else if data_path_from_cfg != "" {
        data_path = check_slash(&data_path_from_cfg);
    }

    if collect != "" && config.collection.is_some() {
        let collectsplit = collect.split(",");
        let collectargs: Vec<&str> = collectsplit.collect();
        let collection = config.collection.as_ref().unwrap();

        for source in collectargs {
            match source {
                "prow" => {
                    if collection.prow.is_some() {
                        let prow = collection.prow.as_ref().unwrap();
                        let location = prow
                            .location
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>();

                        for item in location {
                            if !item.is_empty() {
                                download_metadata(item, &data_path, artifacts_collection).await;
                            }
                        }
                    }
                }
                &_ => {
                    println!("{} is not a valid source", source);
                }
            }
        }
    }

    if http_server {
        let data_path_for_server = data_path.clone();
        let build_info_call =
            move |build_info: Query<BuildId>| handler_api_build(build_info, data_path_for_server);
        let app = Router::new()
            .route("/", get(handler))
            .route("/api/build", get(build_info_call))
            .route("/api/compare", get(handler_api_compare))
            .route("/build/:build_id", get(handler_build_id));
        let app = app.fallback(get(handler_404));

        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {}
