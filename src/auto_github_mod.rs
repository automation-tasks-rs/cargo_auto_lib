// auto_github_mod

//! functions to work with github  

use unwrap::unwrap;

/// create new release on Github  
/// return release_id  
/// it needs env variable `export GITHUB_TOKEN=paste_github_personal_authorization_token_here`  
/// <https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token>  
/// async function can be called from sync code:  
/// ```ignore
///   use tokio::runtime::Runtime;  
///   let rt = Runtime::new().unwrap();  
///   rt.block_on(async move {  
///       let release_id =  github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text).await;  
///       upload_asset_to_github_release(&owner, &repo, &release_id, &path_to_file).await;  
///       println!("Asset uploaded.");  
///   });  
/// ```
/// ```ignore
/// Cargo.toml
/// [dependencies]
/// tokio = {version = "1.10.0", features = ["rt","rt-multi-thread","fs"]}  
/// ```
pub async fn github_create_new_release(
    owner: &str,
    repo: &str,
    version: &str,
    name: &str,
    branch: &str,
    body_md_text: &str,
) -> String {
    use octocrab::Octocrab;
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = unwrap!(Octocrab::builder().personal_token(token).build());
    let new_release = unwrap!(
        octocrab
            .repos(owner, repo)
            .releases()
            .create(&format!("v{}", version))
            .target_commitish(branch)
            .name(name)
            .body(body_md_text)
            .draft(false)
            .prerelease(false)
            .send()
            .await
    );
    new_release.id.to_string()
}

/// upload asset to github release  
/// release_upload_url example: <https://uploads.github.com/repos/owner/repo/releases/48127727/assets>  
/// it needs env variable `export GITHUB_TOKEN=paste_github_personal_authorization_token_here`  
/// <https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token>  
/// async function can be called from sync code:  
/// ```ignore
///   use tokio::runtime::Runtime;  
///   let rt = Runtime::new().unwrap();  
///   rt.block_on(async move {  
///       let release_id =  github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text).await;  
///       upload_asset_to_github_release(&owner, &repo, &release_id, &path_to_file).await;  
///       println!("Asset uploaded.");  
///   });  
/// ```
/// ```ignore
/// Cargo.toml
/// [dependencies]
/// tokio = {version = "1.10.0", features = ["rt","rt-multi-thread","fs"]}  
/// ```
pub async fn github_upload_asset_to_release(
    owner: &str,
    repo: &str,
    release_id: &str,
    path_to_file: &str,
) {
    use octocrab::Octocrab;
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = unwrap!(Octocrab::builder().personal_token(token).build());

    println!("path_to_file: {}", path_to_file);
    let file = std::path::Path::new(&path_to_file);
    let file_name = file.file_name().unwrap().to_str().unwrap();

    let release_upload_url = format!(
        "https://uploads.github.com/repos/{owner}/{repo}/releases/{release_id}/assets",
        owner = owner,
        repo = repo,
        release_id = release_id
    );
    let mut release_upload_url = unwrap!(<url::Url as std::str::FromStr>::from_str(
        &release_upload_url
    ));
    release_upload_url.set_query(Some(format!("{}={}", "name", file_name).as_str()));
    println!("upload_url: {}", release_upload_url);
    let file_size = unwrap!(std::fs::metadata(file)).len();
    println!(
        "file_size: {}. It can take some time to upload. Wait...",
        file_size
    );
    let file = unwrap!(tokio::fs::File::open(file).await);
    let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);
    let builder = octocrab
        .request_builder(release_upload_url.as_str(), reqwest::Method::POST)
        .header("Content-Type", "application/octet-stream")
        .header("Content-Length", file_size.to_string());
    let _response = unwrap!(builder.body(body).send().await);
}
