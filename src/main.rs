use std::{
    fs::{self, OpenOptions},
    io::Write,
    process::Command,
};

fn main() {
    let mut comment: usize = 0;
    let mut deleted: usize = 0;

    loop {
        let output = Command::new("./gradlew").arg("check").output().unwrap();

        let mut counter: u16 = 0;

        for line in String::from_utf8(output.stderr).unwrap().lines() {
            if !line.starts_with('/') {
                continue;
            }

            let (path, line_no) = if let Some(res) = line
                .split(' ')
                .next()
                .unwrap()
                .trim_matches(':')
                .split_once(':')
            {
                res
            } else {
                continue;
            };

            let line_no: usize = line_no.parse().expect(line);

            let content = if let Ok(s) = fs::read_to_string(path) {
                s
            } else {
                continue;
            };

            if line_no == 2 || line_no == content.lines().count() {
                let _ = fs::remove_file(path);
                deleted += 1;
                counter += 1;
                continue;
            };

            let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();

            lines[line_no - 1] = format!("// {}", lines[line_no - 1]);

            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)
                .unwrap()
                .write_all(lines.join("\n").as_bytes())
                .unwrap();

            counter += 1;
            comment += 1;
        }

        println!("{counter} errors removed");

        if counter == 0 {
            break;
        }
    }

    println!("Cleaning completed: {comment} lines and {deleted} classes removed")
}
