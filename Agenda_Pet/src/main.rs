use rusqlite::{params, Connection, Result};
mod models;
use models::{BanhoTosa, ler_entrada};

fn main() -> Result<()> {
    // Conectar ao banco SQLite (arquivo local)
    let conn = Connection::open("projeto_rust.db")?;

    // Criando tabela banho e tosa
    conn.execute(
        "CREATE TABLE IF NOT EXISTS banho_tosa (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cpf TEXT NOT NULL,
            celular TEXT NOT NULL,
            nome_pet TEXT NOT NULL,
            motivo TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS consulta (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cpf TEXT NOT NULL,
            celular TEXT NOT NULL,
            nome_pet TEXT NOT NULL,
            motivo TEXT NOT NULL
        )",
        [],
    )?;

    println!("Agendamento de Banho e Tosa");

    let nome = ler_entrada("Nome do cliente: ");
    let cpf = ler_entrada("CPF: ");
    let celular = ler_entrada("Celular: ");
    let nome_pet = ler_entrada("Nome do pet: ");
    let motivo = ler_entrada("Motivo: ");

    let agendamento = BanhoTosa::new(nome, cpf, celular, nome_pet, motivo);

    // Inserindo no banco
    conn.execute(
        "INSERT INTO banho_tosa (nome, cpf, celular, nome_pet, motivo) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            agendamento.nome,
            agendamento.cpf,
            agendamento.celular,
            agendamento.nome_pet,
            agendamento.motivo
        ],
    )?;

    println!("Agendamento salvo com sucesso!");
    println!("{:?}", agendamento);

    Ok(())
}
