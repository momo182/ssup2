use std::io::ErrorKind;
use std::{fs, io::Error};
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct SSHHost {
    pub host: Vec<String>,
    pub host_name: Option<String>,
    pub user: Option<String>,
    pub port: u16,
    pub proxy_command: Option<String>,
    pub host_key_algorithms: Option<String>,
    pub identity_file: Option<String>,
}

impl SSHHost {
    fn new(host: Vec<String>) -> Self {
        SSHHost {
            host,
            port: 22,
            ..Default::default()
        }
    }
}


// Функция, которая паникует при ошибке
pub fn must_parse_ssh_config(path: &str) -> Vec<SSHHost> {
    parse_ssh_config(path).unwrap_or_else(|e| panic!("Failed to parse SSH config: {}", e))
}


// Функция для парсинга SSH-конфигурации из строки
fn parse(input: &str) -> Result<Vec<SSHHost>, Error> {
    let mut ssh_configs = Vec::new();
    let mut ssh_host: Option<SSHHost> = None;
    let mut lexer = Lexer::new(input);

    while let Some(token) = lexer.next() {
        match token.typ {
            TokenType::Host => {
                if let Some(host) = ssh_host.take() {
                    ssh_configs.push(host);
                }
                let host_values = token.val.split_whitespace().map(String::from).collect();
                ssh_host = Some(SSHHost::new(host_values));
            }
            TokenType::HostName => {
                let value = expect_value(&mut lexer)?;
                ssh_host.as_mut().unwrap().host_name = Some(value);
            }
            TokenType::User => {
                let value = expect_value(&mut lexer)?;
                ssh_host.as_mut().unwrap().user = Some(value);
            }
            TokenType::Port => {
                let value = expect_value(&mut lexer)?;
                let port = u16::from_str(&value).map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid port"))?;
                ssh_host.as_mut().unwrap().port = port;
            }
            TokenType::ProxyCommand => {
                let value = expect_value(&mut lexer)?;
                ssh_host.as_mut().unwrap().proxy_command = Some(value);
            }
            TokenType::HostKeyAlgorithms => {
                let value = expect_value(&mut lexer)?;
                ssh_host.as_mut().unwrap().host_key_algorithms = Some(value);
            }
            TokenType::IdentityFile => {
                let value = expect_value(&mut lexer)?;
                ssh_host.as_mut().unwrap().identity_file = Some(value);
            }
            TokenType::Error => {
                return Err(Error::new(ErrorKind::InvalidData, format!("Error at pos {}", token.pos)));
            }
            TokenType::EOF => {
                if let Some(host) = ssh_host.take() {
                    ssh_configs.push(host);
                }
                break;
            }
            _ => {}
        }
    }

    Ok(ssh_configs)
}



// Вспомогательная функция для получения значения после ключа
fn expect_value(lexer: &mut Lexer) -> Result<String, Error> {
    if let Some(next) = lexer.next() {
        if next.typ == TokenType::Value {
            return Ok(next.val);
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Expected value"))
}

// Лексер для разбора входной строки
struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars().peekable(),
            pos: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        // Реализация лексера зависит от формата входных данных.
        // Здесь предполагается, что токены разделены пробелами или новыми строками.
        todo!("Implement lexer logic based on your specific input format");
    }
}

// Типы токенов
#[derive(Debug, PartialEq)]
enum TokenType {
    Host,
    HostName,
    User,
    Port,
    ProxyCommand,
    HostKeyAlgorithms,
    IdentityFile,
    Value,
    Error,
    EOF,
}

// Структура токена
#[derive(Debug)]
struct Token {
    typ: TokenType,
    val: String,
    pos: usize,
}

// Функция для чтения файла и парсинга конфигурации
fn parse_ssh_config(path: &str) -> Result<Vec<SSHHost>, Error> {
    let content = fs::read_to_string(path)?;
    parse(&content)
}
