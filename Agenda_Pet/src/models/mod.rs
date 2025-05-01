use std::io::{self, Write};
use rusqlite::Row;

#[derive(Debug)]
pub struct BanhoTosa {
    pub nome: String,
    pub cpf: String,
    pub celular: String,
    pub nome_pet: String,
    pub motivo: String,
}

impl BanhoTosa {
    pub fn new(nome: String, cpf: String, celular: String, nome_pet: String, motivo: String) -> Self {
        BanhoTosa {
            nome,
            cpf,
            celular,
            nome_pet,
            motivo,
        }
    }

    // Método para criar BanhoTosa a partir de uma linha do banco (rusqlite)
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(BanhoTosa {
            nome: row.get("nome")?,
            cpf: row.get("cpf")?,
            celular: row.get("celular")?,
            nome_pet: row.get("nome_pet")?,
            motivo: row.get("motivo")?,
        })
    }
}

pub fn ler_entrada(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");
    entrada.trim().to_string()
}
