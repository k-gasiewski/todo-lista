use std::env;
use std::fs::{OpenOptions, File, remove_file};
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        pokaz_pomoc();
        return Ok(());
    }

    let polecenie = &args[1];

    match polecenie.as_str() {
        "dodaj" => {
            if args.len() < 3 {
                eprintln!("Użycie: todo dodaj <zadanie>");
                return Ok(());
            }
            // Połącz wszystkie argumenty po "dodaj" w jedno zdanie
            let zadanie = args[2..].join(" ");
            dodaj_zadanie(&zadanie)?;
        }
        "lista" => wyswietl_zadania()?,
        "zrobione" => {
            if args.len() < 3 {
                eprintln!("Użycie: todo zrobione <zadanie>");
                return Ok(());
            }
            let zadanie = args[2..].join(" "); // Obsługuje spacje w nazwach zadań
            oznacz_zadanie_jako_zrobione(&zadanie)?;
        }
        "wyczysc" => wyczysc_zadania()?,
        "pomoc" => pokaz_pomoc(),
        _ => eprintln!("Nieznane polecenie '{}'. Użyj 'todo pomoc', aby zobaczyć dostępne polecenia.", polecenie),
    }

    Ok(())
}

// Dodaj zadanie do listy (list.txt)
fn dodaj_zadanie(zadanie: &str) -> std::io::Result<()> {
    let plik = OpenOptions::new()
        .create(true)
        .append(true)
        .open("list.txt")?;
    let mut writer = BufWriter::new(plik);
    writeln!(writer, "{}", zadanie)?;
    writer.flush()?;
    println!("Dodano zadanie: {}", zadanie);
    Ok(())
}

// Wyświetl wszystkie zadania
fn wyswietl_zadania() -> std::io::Result<()> {
    let sciezka = "list.txt";
    if !Path::new(sciezka).exists() {
        println!("Brak zapisanych zadań.");
        return Ok(());
    }

    let plik = File::open(sciezka)?;
    let reader = BufReader::new(plik);
    
    println!("Lista zadań:");
    for (index, linia) in reader.lines().enumerate() {
        println!("{}. {}", index + 1, linia?);
    }

    Ok(())
}

// Oznacz zadanie jako ukończone i usuń je z listy
fn oznacz_zadanie_jako_zrobione(zadanie: &str) -> std::io::Result<()> {
    let sciezka = "list.txt";
    if !Path::new(sciezka).exists() {
        println!("Brak zadań do usunięcia.");
        return Ok(());
    }

    let plik = File::open(sciezka)?;
    let reader = BufReader::new(plik);
    
    let zadania: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    let nowe_zadania: Vec<String> = zadania.iter()
        .filter(|&linia| linia != zadanie)
        .cloned()
        .collect();

    if nowe_zadania.len() == zadania.len() {
        println!("Nie znaleziono zadania '{}'.", zadanie);
        return Ok(());
    }

    let mut writer = BufWriter::new(File::create(sciezka)?);
    for zadanie in &nowe_zadania {
        writeln!(writer, "{}", zadanie)?;
    }

    println!("Zadanie '{}' ukończone i usunięte.", zadanie);
    Ok(())
}

// Wyczyść całą listę zadań
fn wyczysc_zadania() -> std::io::Result<()> {
    let sciezka = "list.txt";
    if Path::new(sciezka).exists() {
        remove_file(sciezka)?;
        println!("Wszystkie zadania zostały usunięte.");
    } else {
        println!("Brak zadań do usunięcia.");
    }
    Ok(())
}

// Wyświetl dostępne polecenia
fn pokaz_pomoc() {
    println!(
        "Menadżer zadań CLI\n\
        Użycie:\n\
        todo dodaj <zadanie>    - Dodaj nowe zadanie\n\
        todo lista              - Wyświetl wszystkie zadania\n\
        todo zrobione <zadanie> - Oznacz zadanie jako ukończone\n\
        todo wyczysc            - Wyczyść wszystkie zadania\n\
        todo pomoc              - Wyświetl listę poleceń"
    );
}