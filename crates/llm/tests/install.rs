use llm::install::{LlmInstaller, InstallStatus};

#[test]
fn test_llm_installer_status_and_system_check() {
    let installer = LlmInstaller::new();

    // System requirements check should not panic
    let sys_ok = installer.check_system();
    assert!(sys_ok.is_ok(), "System check should not error");

    // Ollama status should return a valid InstallStatus
    let ollama_status = installer.ollama_status();
    assert!(ollama_status.is_ok(), "Ollama status should not error");
    let ollama_status = ollama_status.unwrap();
    assert!(matches!(ollama_status, InstallStatus::Installed | InstallStatus::NotInstalled | InstallStatus::Failed(_)), "Unexpected Ollama status");

    // Model status should return a valid InstallStatus
    let model_status = installer.model_status();
    assert!(model_status.is_ok(), "Model status should not error");
    let model_status = model_status.unwrap();
    assert!(matches!(model_status, InstallStatus::Installed | InstallStatus::NotInstalled | InstallStatus::Failed(_)), "Unexpected model status");
}

// Note: We do NOT run installer.install_ollama() or installer.install_model() in CI tests
// because they would perform real installations and downloads.
// For full integration, run `cmdr install` manually in a dev environment. 