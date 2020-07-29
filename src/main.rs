use std::env::args;

fn main() {
    match args().nth(1_usize) {
        None => print_help_leap(),
        Some(a) => match a.as_ref() {
            "new" => run_leap_new(),
            _ => print_help_leap(),
        },
    }
}

fn run_leap_new() {
    let template_crate_name = args().nth(2_usize);
    let project_name = args().nth(3_usize);
    if template_crate_name.is_none() || project_name.is_none() {
        print_help_leap_new();
        return;
    }

    cargo_leap::app::commands::leap_new(
        template_crate_name.as_ref().unwrap(),
        project_name.as_ref().unwrap(),
    );
}

fn print_help_leap() {
    println!("{}", include_str!("../man/en_us/help_leap.txt"))
}

fn print_help_leap_new() {
    println!("{}", include_str!("../man/en_us/help_leap_new.txt"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true)
    }
}
