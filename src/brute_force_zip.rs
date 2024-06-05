use reqwest;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::{error::Error, io::Read};
use zip;

#[derive(Serialize, Deserialize)]
struct Input {
    zip_url: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    secret: String,
}

struct PasswordGenerator {
    char_set: Vec<char>,
    width: u32,
    max_width: u32,
    nth: usize,
}

impl PasswordGenerator {
    fn nth_password(&self) -> String {
        (0..self.width)
            .rev()
            .map(|i| {
                let div = self.nth / self.char_set.len().pow(i);
                let rem = div % self.char_set.len();

                self.char_set[rem]
            })
            .collect()
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.width > self.max_width {
            return None;
        }
        if self.nth > self.char_set.len().pow(self.width) {
            self.width += 1;
            self.nth = 1;
        }
        self.nth += 1;
        Some(PasswordGenerator::nth_password(&self))
    }
}

pub async fn brute_force_zip(input: String) -> Result<String, Box<dyn Error>> {
    let input = serde_json::from_str::<Input>(&input)?;
    let zip_bytes = reqwest::get(input.zip_url).await?.bytes().await?;

    let mut archive = zip::ZipArchive::new(Cursor::new(zip_bytes))?;

    let char_set: Vec<char> = ['a'..='z', '0'..='9'].into_iter().flatten().collect();

    let mut password_generator = PasswordGenerator {
        char_set: char_set,
        width: 4,
        max_width: 6,
        nth: 0,
    };

    let secret = password_generator.find_map(|pw| {
        match archive.by_name_decrypt("secret.txt", pw.as_bytes()) {
            Ok(mut z) => {
                let mut str = String::new();
                match z.read_to_string(&mut str) {
                    Ok(_) => Some(String::from(str.strip_suffix("\n").unwrap_or(&str))),
                    _ => None,
                }
            }
            Err(_) => None,
        }
    });

    let output = Output {
        secret: secret.unwrap(),
    };

    Ok(serde_json::to_string(&output)?)
}
