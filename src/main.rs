mod api;
mod search {
    pub mod deezer;
}
use std::{collections::HashMap, env};
mod download;
use reqwest::Client as httpclient;

const HELP: &str = "\
downflac - search and download music written purely in rust

USAGE:
    downflac [OPTIONS] [SONG NAME...]

ARGS:
    <SONG NAME>...    Song to search for. All arguments are joined with
                      spaces and used as the search query.
                      If omitted, you are prompted to enter a name.

OPTIONS:
    -h, --help        Print this help and exit

EXAMPLES:
    downflac never gonna give you up
    downflac

PIPELINE:
    1. Matches up to 5 songs on Deezer for your query.
    2. Searches Kuwo (flac.music.hi.cn) for each match.
    3. Prompts you to select a result.
    4. Downloads it to '<title> by <artist> (<album>).<format>'.

Requires network access to api.deezer.com and flac.music.hi.cn.
";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let roots = webpki_root_certs::TLS_SERVER_ROOT_CERTS
        .iter()
        .filter_map(|c| reqwest::Certificate::from_der(c).ok());
    let mayhtc: Result<httpclient, reqwest::Error> =
        httpclient::builder().tls_certs_only(roots).build();
    let htc = match mayhtc {
        Ok(clnt) => clnt,
        Err(e) => {
            eprintln!("There was an error building the http client: {e}");
            return Ok(());
        }
    };
    let argvs: Vec<String> = env::args().collect();
    if argvs.len() > 1 && matches!(argvs[1].as_str(), "-h" | "--help" | "help") {
        print!("{HELP}");
        return Ok(());
    }
    let mut name: String = String::new();
    if argvs.len() > 1 {
        name = argvs[1..].to_vec().join(" ");
    } else {
        let asname = inquire::Text::new("Enter the name of your intended song:").prompt();
        match asname {
            Ok(nm) => name = nm,
            Err(e) => println!("An error Happend: {}", e),
        }
    }
    let mut opts: Vec<String> = Vec::new();

    let mut mhash: HashMap<String, api::Mdata> = HashMap::new();
    let nusics: Vec<api::Nusic> = api::getmdata(htc.clone(), name)
        .await
        .expect("there was an error searching for your music");
    // let nusics: Vec<deezer::Music> = deezer::dsearch(htc.clone(), name)
    //     .await
    //     .expect("there was an error searching for your music");
    for nsc in nusics {
        let nm: String = format!("{} by {} ({})", nsc.title, nsc.artist, nsc.album);
        mhash.insert(nm.clone(), nsc.data);
        opts.push(nm);
    }
    let nmsc: Result<String, inquire::InquireError> =
        inquire::Select::new("select a song", opts).prompt();
    match nmsc {
        Ok(choice) => {
            let dt: api::Mdata = mhash[&choice].clone();
            let link: String = api::getmusic(htc.clone(), dt.clone()).await?;
            let down: Result<(), Box<dyn std::error::Error>> =
                download::download(link, format!("{}.{}", choice, dt.minfo.format));
            match down {
                Ok(_) => println!("Download Completed"),
                Err(e) => println!("Error: {}", e),
            }
            println!("{}", dt.pic);
            let embed: Result<(), Box<dyn std::error::Error>> =
                download::embed_cover(format!("{}.{}", choice, dt.minfo.format), dt.pic);
            match embed {
                Ok(_) => println!("Embed Completed"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("there was an error: {}", e),
    }
    Ok(())
}
