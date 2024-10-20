use self_github_update::cargo_crate_version;

pub(super) fn update() -> crate::Result<()> {
    use self_github_update::update::UpdateStatus;
    let status = self_github_update::backends::github::Update::configure()
        .repo_owner("penumbra-x")
        .repo_name("duckai")
        .bin_name("duckai")
        .target(self_github_update::get_target())
        .show_output(true)
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update_extended()?;
    if let UpdateStatus::Updated(ref release) = status {
        if let Some(body) = &release.body {
            if !body.trim().is_empty() {
                println!("duckai upgraded to {}:\n", release.version);
                println!("{}", body);
            } else {
                println!("duckai upgraded to {}", release.version);
            }
        }
    } else {
        println!("duckai is up-to-date");
    }

    Ok(())
}
