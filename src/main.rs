#[macro_use]
extern crate clap;
extern crate git2;
extern crate colored;

use git2::*;
use colored::Colorize;

fn main() {
    let matches = clap_app!(myapp =>
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg HASH: +required "Set base commit hash")
            (@arg nth:  +required "Set commit count")
            (@arg PATH: -p --path +takes_value "git repository path")
        ).get_matches();

    let path = matches.value_of("PATH").unwrap_or(".");
    let nth = match matches.value_of("nth").unwrap().parse::<i64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{} nth parse error {}", "error:".red(), e);
            panic!();
        }
    };


    macro_rules! filter_try {
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

    match Repository::open(path) {
        Err(err) => {
            eprintln!("{} {}", "error:".red(), err.message());
        },

        Ok(repo) => {
            println!("Open repository at {}", repo.path().to_str().unwrap());
            let mut revwalk = repo.revwalk().unwrap();
            revwalk.set_sorting(Sort::TIME).unwrap(); 
            revwalk.push_head().unwrap();

            let mut revwalk = revwalk.filter_map(|id|  {
                    let id = filter_try!(id);
                    let commit = filter_try!(repo.find_commit(id));
                    return Some(commit);
                }
            );
            let hash = Oid::from_str(matches.value_of("HASH").unwrap()).unwrap();

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
                        let id = filter_try!(id);
                        let commit = filter_try!(repo.find_commit(id));
                        return Some(commit);
                    }
                );
                let revec = revwalk.collect::<Vec<_>>();
                let target_index = (base_index as i64 + nth) as usize;
                let target_hash = revec[target_index].id();

                println!("target commit hash is {}", target_hash);
            }
        }
    }
}

