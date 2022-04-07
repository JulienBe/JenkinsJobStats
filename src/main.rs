extern crate jenkins_api;

use std::env;
use jenkins_api::{Jenkins, JenkinsBuilder};
use jenkins_api::build::{BuildStatus, CommonBuild};
use jenkins_api::job::WorkflowJob;

#[derive(Debug)]
#[allow(dead_code)]
struct BuildInfo {
    id: u32,
    success: bool,
    time: i64, // for some reason
}

fn main() {
    // meh, it's a cron, excepting nothing more, nothing less then:
    // - user
    // - password
    // - url: http://localhost:8080
    // - job name
    let args: Vec<String> = env::args().collect();
    let user = &args[1];
    let pass = &args[2];
    let url = &args[3];
    let job = &args[4];

    let jenkins = match JenkinsBuilder::new(url).with_user(user, Some(pass)).build() {
        Ok(j) => j,
        Err(error) => panic!("Problem connecting to jenkins: {:?}", error),
    };

    let builds_to_analyze = get_build_numbers(job, &jenkins);
    let builds_info: Vec<BuildInfo> = builds_to_analyze.iter()
        .map(|build_number| get_build(job, *build_number, &jenkins))
        .filter_map(|b| b)
        .map(|b| {
            BuildInfo {
                id: b.number,
                success: if let Some(result) = b.result {
                    result == BuildStatus::Success
                } else {
                    false
                },
                time: b.duration,
            }
        })
        .collect();

    for b in builds_info {
        println!("{:?}", b);
    }
}

fn get_build(job: &String, build_number: u32, jenkins: &Jenkins) -> Option<CommonBuild> {
    match jenkins.get_build(job, build_number) {
        Ok(b) => Some(b),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}
fn get_build_numbers(job: &String, jenkins: &Jenkins) -> Vec<u32> {
    let job = jenkins.get_job(job).unwrap().as_variant::<WorkflowJob>().unwrap();
    job.builds.iter().map(|b| b.number).collect()
}
