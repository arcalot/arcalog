use crate::identification::collect_events;
use crate::system::*;
use async_recursion::async_recursion;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::vec;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

/// Root struct from Prow jobs metadata containing a list of all recent jobs
pub struct Root {
    pub items: Vec<Item>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Item struct containing each individual Prow job's metadata
pub struct Item {
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub metadata: Metadata,
    pub spec: Spec,
    /// Status of each job
    pub status: Status,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
    pub uid: String,
    #[serde(rename = "resourceVersion")]
    pub resource_version: String,
    pub generation: i64,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    pub labels: Labels,
    pub annotations: Annotations,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Labels {
    #[serde(rename = "ci.openshift.io/role")]
    pub ci_openshift_io_role: Option<String>,
    #[serde(rename = "created-by-prow")]
    pub created_by_prow: Option<String>,
    #[serde(rename = "prow.k8s.io/build-id")]
    pub prow_k8s_io_build_id: Option<String>,
    #[serde(rename = "prow.k8s.io/context")]
    pub prow_k8s_io_context: Option<String>,
    #[serde(rename = "prow.k8s.io/id")]
    pub prow_k8s_io_id: Option<String>,
    /// Job type
    #[serde(rename = "prow.k8s.io/job")]
    pub prow_k8s_io_job: String,
    #[serde(rename = "prow.k8s.io/type")]
    pub prow_k8s_io_type: String,
    #[serde(rename = "preset-dind-enabled")]
    pub preset_dind_enabled: Option<String>,
    #[serde(rename = "preset-kind-volume-mounts")]
    pub preset_kind_volume_mounts: Option<String>,
    #[serde(rename = "prow.k8s.io/refs.base_ref")]
    pub prow_k8s_io_refs_base_ref: Option<String>,
    #[serde(rename = "prow.k8s.io/refs.org")]
    pub prow_k8s_io_refs_org: Option<String>,
    #[serde(rename = "prow.k8s.io/refs.repo")]
    pub prow_k8s_io_refs_repo: Option<String>,
    #[serde(rename = "preset-service-account")]
    pub preset_service_account: Option<String>,
    #[serde(rename = "preset-azure-cred")]
    pub preset_azure_cred: Option<String>,
    #[serde(rename = "preset-azure-windows")]
    pub preset_azure_windows: Option<String>,
    #[serde(rename = "preset-k8s-ssh")]
    pub preset_k8s_ssh: Option<String>,
    #[serde(rename = "preset-windows-private-registry-cred")]
    pub preset_windows_private_registry_cred: Option<String>,
    #[serde(rename = "preset-windows-repo-list-master")]
    pub preset_windows_repo_list_master: Option<String>,
    #[serde(rename = "preset-e2e-cos-containerd")]
    pub preset_e2e_cos_containerd: Option<String>,
    #[serde(rename = "preset-aws-credential")]
    pub preset_aws_credential: Option<String>,
    #[serde(rename = "preset-aws-ssh")]
    pub preset_aws_ssh: Option<String>,
    #[serde(rename = "event-GUID")]
    pub event_guid: Option<String>,
    #[serde(rename = "prow.k8s.io/is-optional")]
    pub prow_k8s_io_is_optional: Option<String>,
    #[serde(rename = "prow.k8s.io/refs.pull")]
    pub prow_k8s_io_refs_pull: Option<String>,
    #[serde(rename = "created-by-tide")]
    pub created_by_tide: Option<String>,
    #[serde(rename = "preset-use-new-registry")]
    pub preset_use_new_registry: Option<String>,
    #[serde(rename = "preset-bazel-remote-cache-enabled")]
    pub preset_bazel_remote_cache_enabled: Option<String>,
    #[serde(rename = "preset-bazel-scratch-dir")]
    pub preset_bazel_scratch_dir: Option<String>,
    #[serde(rename = "preset-common-gce-windows")]
    pub preset_common_gce_windows: Option<String>,
    #[serde(rename = "preset-e2e-gce-windows")]
    pub preset_e2e_gce_windows: Option<String>,
    #[serde(rename = "preset-e2e-gce-windows-containerd")]
    pub preset_e2e_gce_windows_containerd: Option<String>,
    #[serde(rename = "preset-e2e-scalability-periodics")]
    pub preset_e2e_scalability_periodics: Option<String>,
    #[serde(rename = "preset-e2e-scalability-periodics-master")]
    pub preset_e2e_scalability_periodics_master: Option<String>,
    #[serde(rename = "preset-e2e-scalability-node")]
    pub preset_e2e_scalability_node: Option<String>,
    #[serde(rename = "preset-e2e-kubemark-common")]
    pub preset_e2e_kubemark_common: Option<String>,
    #[serde(rename = "preset-azure-anonymous-pull")]
    pub preset_azure_anonymous_pull: Option<String>,
    #[serde(rename = "preset-azure-cred-only")]
    pub preset_azure_cred_only: Option<String>,
    #[serde(rename = "preset-e2e-containerd")]
    pub preset_e2e_containerd: Option<String>,
    #[serde(rename = "preset-e2e-containerd-image-load")]
    pub preset_e2e_containerd_image_load: Option<String>,
    pub app: Option<String>,
    #[serde(rename = "preset-e2e-scalability-common")]
    pub preset_e2e_scalability_common: Option<String>,
    #[serde(rename = "preset-ci-gce-device-plugin-gpu")]
    pub preset_ci_gce_device_plugin_gpu: Option<String>,
    #[serde(rename = "preset-e2e-scalability-presubmits")]
    pub preset_e2e_scalability_presubmits: Option<String>,
    #[serde(rename = "prow.k8s.io/retest")]
    /// If the value is "true", this job was a retest of a previous run that failed
    pub prow_k8s_io_retest: Option<String>,
    #[serde(rename = "preset-do-credential")]
    pub preset_do_credential: Option<String>,
    #[serde(rename = "preset-pull-kubernetes-e2e")]
    pub preset_pull_kubernetes_e2e: Option<String>,
    #[serde(rename = "preset-pull-kubernetes-e2e-gce")]
    pub preset_pull_kubernetes_e2e_gce: Option<String>,
    #[serde(rename = "preset-capz-containerd-latest")]
    pub preset_capz_containerd_latest: Option<String>,
    #[serde(rename = "preset-capz-windows-2019")]
    pub preset_capz_windows_2019: Option<String>,
    #[serde(rename = "preset-capz-windows-common-main")]
    pub preset_capz_windows_common_main: Option<String>,
    #[serde(rename = "preset-capz-windows-parallel")]
    pub preset_capz_windows_parallel: Option<String>,
    #[serde(rename = "preset-do-spaces-credential")]
    pub preset_do_spaces_credential: Option<String>,
    #[serde(rename = "preset-do-ssh")]
    pub preset_do_ssh: Option<String>,
    #[serde(rename = "preset-capz-windows-common-124")]
    pub preset_capz_windows_common_124: Option<String>,
    #[serde(rename = "preset-ingress-master-yaml")]
    pub preset_ingress_master_yaml: Option<String>,
    #[serde(rename = "preset-cluster-api-provider-vsphere-e2e-config")]
    pub preset_cluster_api_provider_vsphere_e2e_config: Option<String>,
    #[serde(rename = "preset-cluster-api-provider-vsphere-gcs-creds")]
    pub preset_cluster_api_provider_vsphere_gcs_creds: Option<String>,
    #[serde(rename = "preset-aws-credential-aws-oss-testing")]
    pub preset_aws_credential_aws_oss_testing: Option<String>,
    #[serde(rename = "preset-e2e-kubemark-gce-scale")]
    pub preset_e2e_kubemark_gce_scale: Option<String>,
    #[serde(rename = "preset-azure-secrets-store-creds")]
    pub preset_azure_secrets_store_creds: Option<String>,
    #[serde(rename = "preset-capz-windows-common-pull")]
    pub preset_capz_windows_common_pull: Option<String>,
    #[serde(rename = "preset-cloud-provider-vsphere-e2e-config")]
    pub preset_cloud_provider_vsphere_e2e_config: Option<String>,
    #[serde(rename = "preset-e2e-platform-aws")]
    pub preset_e2e_platform_aws: Option<String>,
    #[serde(rename = "preset-capz-serial-slow")]
    pub preset_capz_serial_slow: Option<String>,
    #[serde(rename = "preset-capz-windows-azuredisk")]
    pub preset_capz_windows_azuredisk: Option<String>,
    #[serde(rename = "preset-capz-windows-ci-entrypoint-common-main")]
    pub preset_capz_windows_ci_entrypoint_common_main: Option<String>,
    #[serde(rename = "release.openshift.io/payload")]
    pub release_openshift_io_payload: Option<String>,
    #[serde(rename = "release.openshift.io/verify")]
    pub release_openshift_io_verify: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotations {
    pub description: Option<String>,
    #[serde(rename = "prow.k8s.io/context")]
    pub prow_k8s_io_context: Option<String>,
    #[serde(rename = "prow.k8s.io/job")]
    pub prow_k8s_io_job: String,
    #[serde(rename = "testgrid-dashboards")]
    pub testgrid_dashboards: Option<String>,
    #[serde(rename = "testgrid-tab-name")]
    pub testgrid_tab_name: Option<String>,
    #[serde(rename = "testgrid-alert-email")]
    pub testgrid_alert_email: Option<String>,
    #[serde(rename = "testgrid-alert-stale-results-hours")]
    pub testgrid_alert_stale_results_hours: Option<String>,
    #[serde(rename = "testgrid-num-failures-to-alert")]
    pub testgrid_num_failures_to_alert: Option<String>,
    #[serde(rename = "fork-per-release-periodic-interval")]
    pub fork_per_release_periodic_interval: Option<String>,
    #[serde(rename = "fork-per-release")]
    pub fork_per_release: Option<String>,
    #[serde(rename = "testgrid-create-test-group")]
    pub testgrid_create_test_group: Option<String>,
    #[serde(rename = "testgrid-days-of-results")]
    pub testgrid_days_of_results: Option<String>,
    #[serde(rename = "test.kops.k8s.io/cloud")]
    pub test_kops_k8s_io_cloud: Option<String>,
    #[serde(rename = "test.kops.k8s.io/container_runtime")]
    pub test_kops_k8s_io_container_runtime: Option<String>,
    #[serde(rename = "test.kops.k8s.io/distro")]
    pub test_kops_k8s_io_distro: Option<String>,
    #[serde(rename = "test.kops.k8s.io/k8s_version")]
    pub test_kops_k8s_io_k8s_version: Option<String>,
    #[serde(rename = "test.kops.k8s.io/kops_channel")]
    pub test_kops_k8s_io_kops_channel: Option<String>,
    #[serde(rename = "test.kops.k8s.io/kops_version")]
    pub test_kops_k8s_io_kops_version: Option<String>,
    #[serde(rename = "test.kops.k8s.io/networking")]
    pub test_kops_k8s_io_networking: Option<String>,
    #[serde(rename = "testgrid-num-columns-recent")]
    pub testgrid_num_columns_recent: Option<String>,
    #[serde(rename = "fork-per-release-cron")]
    pub fork_per_release_cron: Option<String>,
    #[serde(rename = "fork-per-release-deletions")]
    pub fork_per_release_deletions: Option<String>,
    #[serde(rename = "fork-per-release-replacements")]
    pub fork_per_release_replacements: Option<String>,
    #[serde(rename = "test.kops.k8s.io/extra_flags")]
    pub test_kops_k8s_io_extra_flags: Option<String>,
    #[serde(rename = "testgrid-broken-column-threshold")]
    pub testgrid_broken_column_threshold: Option<String>,
    #[serde(rename = "testgrid-base-options")]
    pub testgrid_base_options: Option<String>,
    #[serde(rename = "test-grid-alert-email")]
    pub test_grid_alert_email: Option<String>,
    #[serde(rename = "test.kops.k8s.io/feature_flags")]
    pub test_kops_k8s_io_feature_flags: Option<String>,
    #[serde(rename = "testgrid-description")]
    pub testgrid_description: Option<String>,
    #[serde(rename = "ci.openshift.io/description")]
    pub ci_openshift_io_description: Option<String>,
    #[serde(rename = "ci.openshift.io/sop")]
    pub ci_openshift_io_sop: Option<String>,
    #[serde(rename = "release.openshift.io/architecture")]
    pub release_openshift_io_architecture: Option<String>,
    #[serde(rename = "release.openshift.io/source")]
    pub release_openshift_io_source: Option<String>,
    #[serde(rename = "release.openshift.io/tag")]
    pub release_openshift_io_tag: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    #[serde(rename = "type")]
    pub type_field: String,
    pub agent: String,
    pub cluster: String,
    pub namespace: String,
    pub job: String,
    pub report: Option<bool>,
    pub pod_spec: Option<PodSpec>,
    pub decoration_config: Option<DecorationConfig>,
    pub prowjob_defaults: Option<ProwjobDefaults>,
    pub rerun_auth_config: Option<RerunAuthConfig>,
    pub max_concurrency: Option<i64>,
    #[serde(default)]
    pub extra_refs: Vec<ExtraRef>,
    pub refs: Option<Refs>,
    pub context: Option<String>,
    pub rerun_command: Option<String>,
    pub reporter_config: Option<ReporterConfig>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodSpec {
    #[serde(default)]
    pub volumes: Vec<Volume>,
    pub containers: Vec<Container>,
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext2>,
    #[serde(rename = "nodeSelector")]
    pub node_selector: Option<NodeSelector>,
    #[serde(default)]
    pub tolerations: Vec<Toleration>,
    #[serde(rename = "hostNetwork")]
    pub host_network: Option<bool>,
    #[serde(rename = "hostPID")]
    pub host_pid: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    pub secret: Option<Secret>,
    #[serde(rename = "emptyDir")]
    pub empty_dir: Option<EmptyDir>,
    #[serde(rename = "hostPath")]
    pub host_path: Option<HostPath>,
    #[serde(rename = "configMap")]
    pub config_map: Option<ConfigMap>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(rename = "defaultMode")]
    pub default_mode: Option<i64>,
    #[serde(default)]
    pub items: Vec<Item2>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item2 {
    pub key: String,
    pub path: String,
    pub mode: Option<i64>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyDir {}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostPath {
    pub path: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigMap {
    pub name: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<Vec<Env>>,
    pub resources: Option<Resources>,
    #[serde(rename = "volumeMounts")]
    #[serde(default)]
    pub volume_mounts: Vec<VolumeMount>,
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext>,
    #[serde(rename = "envFrom")]
    #[serde(default)]
    pub env_from: Vec<EnvFrom>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Env {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "valueFrom")]
    pub value_from: Option<ValueFrom>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: Option<SecretKeyRef>,
    #[serde(rename = "fieldRef")]
    pub field_ref: Option<FieldRef>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretKeyRef {
    pub name: String,
    pub key: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldRef {
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resources {
    pub requests: Option<Requests>,
    pub limits: Option<Limits>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requests {
    pub cpu: Option<String>,
    pub memory: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    pub memory: String,
    pub cpu: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityContext {
    pub privileged: bool,
    pub capabilities: Option<Capabilities>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capabilities {
    pub add: Vec<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvFrom {
    #[serde(rename = "secretRef")]
    pub secret_ref: SecretRef,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretRef {
    pub name: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityContext2 {
    #[serde(rename = "runAsUser")]
    pub run_as_user: i64,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSelector {
    pub highcpu: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Toleration {
    pub key: String,
    pub operator: String,
    pub value: String,
    pub effect: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecorationConfig {
    pub timeout: String,
    pub grace_period: String,
    pub utility_images: UtilityImages,
    pub resources: Option<Resources2>,
    pub gcs_configuration: GcsConfiguration,
    pub gcs_credentials_secret: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilityImages {
    pub clonerefs: String,
    pub initupload: String,
    pub entrypoint: String,
    pub sidecar: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resources2 {
    pub clonerefs: Clonerefs,
    pub initupload: Initupload,
    pub place_entrypoint: PlaceEntrypoint,
    pub sidecar: Sidecar,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clonerefs {
    pub requests: Requests2,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requests2 {
    pub cpu: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Initupload {
    pub requests: Requests3,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requests3 {
    pub cpu: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceEntrypoint {
    pub requests: Requests4,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requests4 {
    pub cpu: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sidecar {
    pub requests: Requests5,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requests5 {
    pub cpu: String,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GcsConfiguration {
    pub bucket: String,
    pub path_strategy: String,
    pub default_org: String,
    pub default_repo: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProwjobDefaults {
    pub tenant_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RerunAuthConfig {
    #[serde(default)]
    pub github_team_slugs: Vec<GithubTeamSlug>,
    #[serde(default)]
    pub github_users: Vec<String>,
    #[serde(default)]
    pub github_team_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubTeamSlug {
    pub slug: String,
    pub org: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraRef {
    pub org: String,
    pub repo: String,
    pub base_ref: String,
    pub path_alias: Option<String>,
    pub workdir: Option<bool>,
    pub base_sha: Option<String>,
    pub clone_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refs {
    pub org: String,
    pub repo: String,
    pub repo_link: Option<String>,
    pub base_ref: String,
    pub base_sha: Option<String>,
    pub base_link: Option<String>,
    #[serde(default)]
    pub pulls: Vec<Pull>,
    pub path_alias: Option<String>,
    pub skip_submodules: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pull {
    pub number: i64,
    pub author: String,
    pub sha: String,
    pub title: String,
    pub link: Option<String>,
    pub commit_link: Option<String>,
    pub author_link: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReporterConfig {
    pub slack: Slack,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slack {
    pub channel: String,
    pub job_states_to_report: Vec<String>,
    pub report_template: Option<String>,
    pub report: bool,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Individual job status metadata
pub struct Status {
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "pendingTime")]
    pub pending_time: Option<String>,
    #[serde(rename = "completionTime")]
    pub completion_time: Option<String>,
    /// Indicates job state, including `failure`, `pending`, `success`, and more as defined in Prow
    pub state: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub pod_name: Option<String>,
    pub build_id: Option<String>,
    pub prev_report_states: Option<PrevReportStates>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrevReportStates {
    pub gcsk8sreporter: Option<String>,
    pub gcsreporter: Option<String>,
    #[serde(rename = "github-reporter")]
    pub github_reporter: Option<String>,
    pub slackreporter: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Job types with a link to all their build IDs
pub struct JobTypeBuilds {
    pub job: String,
    pub builds: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildInfo {
    pub build_id: String,
    pub build_url: Option<String>,
    pub label: Option<String>,
    pub state: Option<String>,
    pub job_type: Option<String>,
    pub events: Option<Vec<String>>,
    pub error: Option<String>,
}

/// Downloads the latest Prow jobs metadata and creates searchable files
///
///  # Arguments
///
/// * `location` - The URL from which Prow metadata needs to be downloaded
/// * `volume` - The data path where the files will be stored
/// * `artifactions_collection` - If set to true, the function will also download all artifacts of all builds in the current collection step
pub async fn download_metadata(location: &str, volume: &str, artifacts_collection: bool) {
    let time_id = add_time_id();
    let volume_slash = check_slash(&volume);
    let folder_data = format!("{}prow", &volume_slash);
    let folder_success = format!("{}prow/success", &volume_slash);
    let folder_failure = format!("{}prow/failure", &volume_slash);
    let folder_type = format!("{}prow/type", &volume_slash);

    if !Path::new(&folder_data).exists() {
        std::fs::create_dir_all(&folder_data)
            .expect(format!("Failed to create directory: {}", &folder_data).as_str());
    }
    if !Path::new(&folder_success).exists() {
        std::fs::create_dir_all(&folder_success)
            .expect(format!("Failed to create directory: {}", &folder_success).as_str());
    }
    if !Path::new(&folder_failure).exists() {
        std::fs::create_dir_all(&folder_failure)
            .expect(format!("Failed to create directory: {}", &folder_failure).as_str());
    }
    if !Path::new(&folder_type).exists() {
        std::fs::create_dir_all(&folder_type)
            .expect(format!("Failed to create directory: {}", &folder_type).as_str());
    }

    let path = format!("{}prow/collect-{}.json", &volume_slash, &time_id);
    let collect_url = check_slash(&location);
    let items: Root = reqwest::get(format!("{}prowjobs.js", collect_url).as_str())
        .await
        .expect("Failed to download Prow metadata")
        .json()
        .await
        .expect("Failed to parse Prow metadata");
    serde_json::to_writer_pretty(&File::create(&path).expect("Failed to create file"), &items)
        .expect("Failed to write JSON to file");

    let mut build_id_failures = HashMap::new();
    let mut build_id_successes = HashMap::new();
    let mut job_types = HashMap::new();

    for item in &items.items {
        let build_id = match item.status.build_id.as_ref() {
            Some(build_id) => build_id,
            None => "",
        };
        let job_type = item.metadata.labels.prow_k8s_io_job.as_str();

        if !job_types.contains_key(&job_type) {
            job_types.insert(job_type, Vec::new());
        }

        if !job_types.get(&job_type).unwrap().contains(&build_id) {
            job_types.get_mut(&job_type).unwrap().push(build_id.clone());
        }

        let state = match item.status.state.as_ref() {
            Some(state) => state,
            None => "",
        };

        if state == "success" {
            build_id_successes.insert(
                build_id,
                [
                    item.status.url.as_ref().unwrap().to_string(),
                    item.metadata.labels.prow_k8s_io_job.to_string(),
                ],
            );
        } else if state == "failure" {
            build_id_failures.insert(
                build_id,
                [
                    item.status.url.as_ref().unwrap().to_string(),
                    item.metadata.labels.prow_k8s_io_job.to_string(),
                ],
            );
        }
    }
    let path = format!("{}prow/failure/builds-{}.json", &volume_slash, &time_id);
    serde_json::to_writer_pretty(
        &File::create(path).expect("Failed to create job failure file"),
        &build_id_failures,
    )
    .expect("Failed to write failure map to file");

    let path = format!("{}prow/success/builds-{}.json", &volume_slash, &time_id);
    serde_json::to_writer_pretty(
        &File::create(path).expect("Failed to create job success file"),
        &build_id_successes,
    )
    .expect("Failed to write success map to file");

    let path = format!("{}prow/type/types-{}.json", &volume_slash, &time_id);
    serde_json::to_writer_pretty(
        &File::create(path).expect("Failed to create job types file"),
        &job_types,
    )
    .expect("Failed to write job types map to file");

    if artifacts_collection {
        for item in build_id_failures {
            download_artifacts(item.0, &volume_slash).await;
        }

        for item in build_id_successes {
            download_artifacts(item.0, &volume_slash).await;
        }
    }
}

/// Downloads the latest artifacts from Prow for the **given build ID**
///
///  # Arguments
///
/// * `build_id` - The build ID for which artifacts need to be downloaded
/// * `path` - String representing the path where the artifacts and metadata are stored
pub async fn download_artifacts(build_id: &str, path: &str) {
    let mut build_link: String = String::from("");
    let path_slash = check_slash(path);
    let failure_files = files_in_folder(format!("{}prow/failure", path_slash).as_str()).await;
    let success_files = files_in_folder(format!("{}prow/success", path_slash).as_str()).await;
    let folder_artifacts = format!("{}prow/artifacts", path_slash);
    if !Path::new(&folder_artifacts).exists() {
        std::fs::create_dir_all(&folder_artifacts)
            .expect(format!("Failed to create directory: {}", &folder_artifacts).as_str());
    }

    if success_files.is_empty() && failure_files.is_empty() {
        println!(
            "⛔\t\x1b[93m\x1b[1mNo data found. Please run at least one collection step first.\x1b[0m"
        );
    } else {
        for file in failure_files {
            let mut collect_file = File::open(&file).expect("Failed to open failure file");
            let check_file: HashMap<String, [String; 2]> =
                serde_json::from_reader(&mut collect_file)
                    .expect("Failed to deserialize file contents");
            build_link = match check_file.get(build_id) {
                Some(url) => url[0].clone(),
                None => String::from(""),
            };
        }
        if build_link == "" {
            for file in success_files {
                let mut collect_file = File::open(&file).expect("Failed to open success file");
                let check_file: HashMap<String, [String; 2]> =
                    serde_json::from_reader(&mut collect_file)
                        .expect("Failed to deserialize file contents");
                build_link = match check_file.get(build_id) {
                    Some(url) => url[0].clone(),
                    None => String::from(""),
                };
            }
        }
        if build_link == "" {
            println!(
                "⛔\t\x1b[93m\x1b[1mBuild ID not found: {}\x1b[0m",
                &build_id
            );
        }
        if build_link != "" {
            for url in find_urls(&build_link, "Artifacts".to_string()).await {
                let artifact_path = format!("{}prow/artifacts/{}", &path_slash, &build_id);
                download_artifacts_recursive(&url, &artifact_path).await;
            }
        }
    }
}

/// Recursively downloads all artifacts for a **given url into the given path**
///
///  # Arguments
///
/// * `url` - The initial URL from where artifacts should be downloaded
/// * `target_path` - Path to where the artifacts are stored, mirroring the existing artifact folder structure
#[async_recursion]
pub async fn download_artifacts_recursive(url: &str, target_path: &str) {
    let base_url_split = url.split("/");
    let base_url = base_url_split.collect::<Vec<&str>>()[0..3].join("/");

    for artifact_url in find_urls(&url, "".to_string()).await {
        let mut absolute_artifact_url = artifact_url.to_string();
        if artifact_url.starts_with("/") {
            absolute_artifact_url = format!("{}{}", &base_url, &artifact_url);
        }
        if &absolute_artifact_url.len() < &url.len() {
            continue;
        }
        let artifact_name = &artifact_url
            .rsplit_terminator("/")
            .next()
            .unwrap()
            .to_string();
        if artifact_name.eq("..") || artifact_name.eq(".") {
            continue;
        }
        if !Path::new(&target_path).exists() {
            std::fs::create_dir_all(&target_path)
                .expect(format!("Failed to create directory: {}", &target_path).as_str());
        }
        if artifact_url.ends_with("/") {
            println!(
                "📁\t\x1b[32m\x1b[1mDownloading artifact: {}\x1b[0m",
                &absolute_artifact_url
            );
            download_artifacts_recursive(
                &absolute_artifact_url,
                format!("{}/{}", &target_path, &artifact_name).as_str(),
            )
            .await;
        } else {
            let target_file = format!("{}/{}", &target_path, &artifact_name);
            if !Path::new(&target_file).exists() {
                let resp = reqwest::get(&absolute_artifact_url)
                    .await
                    .expect("Failed to get artifact")
                    .bytes()
                    .await
                    .expect("Failed to get artifact contents");
                File::create(&target_file).expect("Failed to create artifact file");
                std::fs::write(&target_file, resp.as_ref()).expect("Failed to write artifact file");
            }
        }
    }
}

/// Find URLs in a text using a regular expression searching for HTML tags
///
/// # Arguments
///
/// * `aname` - The name (text description) between the HTML anchor tags
///
/// # Returns
///
/// A vector of URLs found in the text
pub async fn find_urls(url: &str, aname: String) -> Vec<String> {
    let resp = reqwest::get(url)
        .await
        .expect("Failed to get HTML from given URL")
        .text()
        .await
        .expect("Failed to convert HTML to text");

    if &aname == "" {
        let mut urls = Vec::new();
        let rx = Regex::new("<a .*?href=\"([^\"]*)\">(.*?)</a>").unwrap();
        for cap in rx.captures_iter(&resp) {
            urls.push(cap[1].to_string());
        }

        return urls;
    } else {
        let aname_str = &aname.as_str();
        let rx =
            Regex::new(format!("<a .*?href=\"([^\"]*)\">(.*?){}</a>", aname_str).as_str()).unwrap();
        let captures = rx.captures(&resp).unwrap();
        return vec![captures[1].to_string()];
    }
}

/// Get build information for a given build ID
///
/// # Arguments
///
/// * `build_id` - Request build ID
/// * `source_path` - Root path to where the build data is stored
///
/// # Returns
///
/// `BuildInfo` - Struct containing the build information
pub async fn get_build_info(build_id: String, source_path: String) -> BuildInfo {
    if build_id == "" {
        return BuildInfo {
            build_id: build_id,
            build_url: None,
            label: None,
            state: None,
            job_type: None,
            events: None,
            error: Some("Have you forgotten to submit a build ID?".to_string()),
        };
    } else {
        let mut send_build_info = BuildInfo {
            build_id: build_id.clone(),
            build_url: None,
            label: None,
            state: None,
            job_type: None,
            events: None,
            error: None,
        };
        let index_failure = create_file_index(format!("{}/prow/failure", source_path)).await;
        let index_success = create_file_index(format!("{}/prow/success", source_path)).await;
        for filename in index_failure {
            let mut open_file = File::open(&filename).expect("Failed to open file");
            let file_contents: HashMap<String, Vec<String>> =
                serde_json::from_reader(&mut open_file)
                    .expect("Failed to deserialize file contents");
            if file_contents.contains_key(&build_id) {
                let build_info = file_contents.get(&build_id).unwrap();
                send_build_info.build_url = Some(build_info[0].to_string());
                send_build_info.label = Some("❓ unknown".to_string());
                send_build_info.state = Some("⛔ failure".to_string());
                send_build_info.job_type = Some(build_info[1].to_string());
                break;
            }
        }
        if send_build_info.build_url.is_none() {
            for filename in index_success {
                let mut open_file = File::open(&filename).expect("Failed to open file");
                let file_contents: HashMap<String, Vec<String>> =
                    serde_json::from_reader(&mut open_file)
                        .expect("Failed to deserialize file contents");
                if file_contents.contains_key(&build_id) {
                    let build_info = file_contents.get(&build_id).unwrap();
                    send_build_info.build_url = Some(build_info[0].to_string());
                    send_build_info.label = Some("❓ unknown".to_string());
                    send_build_info.state = Some("✅ success".to_string());
                    send_build_info.job_type = Some(build_info[1].to_string());
                    break;
                }
            }
        }

        if send_build_info.build_url.is_some() {
            let all_events = collect_events(
                build_id.clone(),
                format!("{}prow/artifacts/{}", source_path, &build_id),
            )
            .await;
            send_build_info.events = Some(all_events);
            return send_build_info;
        } else {
            return BuildInfo {
                build_id: build_id.to_string(),
                build_url: None,
                label: None,
                state: None,
                job_type: None,
                events: None,
                error: Some("Please put in a valid build ID".to_string()),
            };
        }
    }
}

#[cfg(test)]
mod tests {}
