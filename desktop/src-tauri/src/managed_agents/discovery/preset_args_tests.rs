use super::normalize_agent_args;

#[test]
fn preset_commands_keep_their_declared_args() {
    // Empty record args must not launch a preset's bare interactive command.
    for command in ["devin", "cursor-agent", "omp", "openclaw"] {
        assert_eq!(
            normalize_agent_args(command, Vec::new()),
            vec!["acp".to_string()]
        );
    }
    assert_eq!(
        normalize_agent_args("grok", Vec::new()),
        vec!["agent", "--always-approve", "stdio"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    );
}

#[test]
fn preset_args_resolve_through_paths_and_runtime_ids() {
    assert_eq!(
        normalize_agent_args("/Users/me/.local/bin/devin", Vec::new()),
        vec!["acp".to_string()]
    );
    assert_eq!(
        normalize_agent_args("cursor", Vec::new()),
        vec!["acp".to_string()]
    );
}

#[test]
fn explicit_preset_args_win_over_declared_defaults() {
    assert_eq!(
        normalize_agent_args("devin", vec!["acp".into(), "--verbose".into()]),
        vec!["acp".to_string(), "--verbose".to_string()]
    );
}

#[test]
fn adapter_presets_stay_argless() {
    assert_eq!(
        normalize_agent_args("amp-acp", Vec::new()),
        Vec::<String>::new()
    );
    assert_eq!(
        normalize_agent_args("amp-acp", vec!["acp".into()]),
        Vec::<String>::new()
    );
    assert_eq!(
        normalize_agent_args("hermes-acp", Vec::new()),
        Vec::<String>::new()
    );
}

#[test]
fn unknown_commands_still_resolve_to_no_args() {
    assert_eq!(
        normalize_agent_args("some-custom-harness", Vec::new()),
        Vec::<String>::new()
    );
    assert_eq!(
        normalize_agent_args("some-custom-harness", vec!["--flag".into()]),
        vec!["--flag".to_string()]
    );
}
