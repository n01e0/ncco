#[macro_use]
extern crate clap;
extern crate git2;
extern crate colored;

use git2::*;
use colored::Colorize;

macro_rules! unwrap_or_exit {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) =>  {
                eprintln!("{} {}", "error:".red(), e.message());
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let matches = clap_app!(ncco =>
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg HASH: +required "Set base commit hash")
            (@arg nth:  +required "Set commit count")
            (@arg PATH: -p --path +takes_value "git repository path(default PWD)")
        ).get_matches();

    let path = matches.value_of("PATH").unwrap_or(".");
    let nth = match matches.value_of("nth").unwrap().parse::<i64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{} <nth> commit count parse error {}", "error:".red(), e);
            std::process::exit(1);
        }
    };


    match Repository::open(path) {
        Err(err) => {
            eprintln!("{} {}", "error:".red(), err.message());
        },

        Ok(repo) => {
            let mut revwalk = repo.revwalk().unwrap();
            revwalk.set_sorting(Sort::TIME).unwrap(); 
            revwalk.push_head().unwrap();

            let mut revwalk = revwalk.filter_map(|id|  {
                    let id = unwrap_or_exit!(id);
                    let commit = unwrap_or_exit!(repo.find_commit(id));
                    return Some(commit);
                }
            );
            let hash = unwrap_or_exit!(Oid::from_str(matches.value_of("HASH").unwrap()));

            if let Ok(_) = repo.find_commit(hash) {
                let base_index = revwalk.position(|c| c.id() == hash).unwrap();

                if base_index == 0 {
                    eprintln!("{} {} is HEAD!!", "error:".red(), hash);
                    std::process::exit(1);
                }

                let mut revwalk = repo.revwalk().unwrap();
                revwalk.set_sorting(Sort::TIME).unwrap(); 
                revwalk.push_head().unwrap();

                let revwalk = revwalk.filter_map(|id|  {
                        let id = unwrap_or_exit!(id);
                        let commit = unwrap_or_exit!(repo.find_commit(id));
                        return Some(commit);
                    }
                );
                let revec = revwalk.collect::<Vec<_>>();
                let target_index = (base_index as i64 + nth) as usize;
                let target_hash = revec[target_index].id();

                println!("target commit hash is {}", target_hash);
            } else {
                eprintln!("{} Commit ID {} not found.", "error".red(), hash);
                std::process::exit(1)
            }
        }
    }
}

