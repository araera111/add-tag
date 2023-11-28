use chrono::Local;

fn main() {
    println!("git-tag-update-start");
    let local = Local::now();
    println!("local: {}", local);
    let today = local.format("%Y-%m-%d").to_string();
    let question = requestty::Question::input("tag_name")
        .message("Please enter the tag name")
        .default(format!("v{}", today))
        .build();
    let answer = requestty::prompt_one(question).unwrap();
    println!("answer: {:#?}", answer);
    println!("git-tag-update-end");

    let str_answer = answer.as_string().unwrap();
    println!("str_answer: {}", str_answer);

    /* command: git tag -a tag */
    let output = std::process::Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(str_answer)
        .output()
        .expect("failed to execute process");
    println!("output: {:#?}", output);

    /* command: git push origin tag */
    let output = std::process::Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(str_answer)
        .output()
        .expect("failed to execute process");
    println!("output: {:#?}", output);
}
