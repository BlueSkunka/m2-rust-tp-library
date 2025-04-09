use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use csv::StringRecordsIter;
use dialoguer::Select;

const LIBRARY_FILE: &str = "library.csv";

// Structure représantant un livre
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    published_year: String
}

// Enumération des actions possibles
enum Action {
    AddBook,
    SearchBook,
    ReadLibrary,
    RemoveBook
}

fn main() -> Result<(), Box<dyn Error>> {
    read_library();

    doctor();

    let action = menu();

    // Suivant l'action choisis, on renvoie vers la méthode qui gère l'action
    match action {
        0 => add_book_action(),
        1 => search_book_action(),
        2 => read_library_action(),
        3 => remove_book_action(),
        _ => println!("Leaving library")
    };

    Ok(())
}

// Assure l'existence du fichier csv
fn doctor() {
    if !Path::new(LIBRARY_FILE).exists() {
        let file = File::create(LIBRARY_FILE);
        println!("Library created")
    }

    println!("Library already up !")
}

// Affiche le menu et renvoie l'action choisie
fn menu() -> usize {
    let options = [
        "-> Ajouter un livre",
        "-> Rechercher un livre",
        "-> Lister les livres",
        "-> Retirer un livre",
        "-> Quitter"
    ];

    let index = Select::new()
        .with_prompt(" ----- Choisir une action -----")
        .items(&options)
        .interact().unwrap();

    println!("Action selected: {}", options[index]);

    index
}

fn add_book_action() {
    println!("Adding book !");

    // Faire saisir les champs à l'utilisateur
    let mut title = String::new();

    io::stdin().read_line(&mut title).expect("Impossible de lire votre saisie !");

    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Impossible de lire saisie !");

    let mut isbn = String::new();
    io::stdin().read_line(&mut isbn).expect("Impossible de lire saisie !");

    let mut published_year = String::new();
    io::stdin().read_line(&mut published_year).expect("Impossible de lire saisie !");

    let book: Book = Book{
        title: title,
        author: author,
        isbn: isbn,
        published_year: published_year
    };

    // lecture des records existant
    if (search_book(&book.title)) {
        println!("Ce livre est déjà en rayon !");
    } else {
        println!("Ajout du livre");
    }


}

// Affiche les infos du livre recherché
fn search_book_action() {
    println!("Quel livre cherchez vous ?");

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Impossible de lire votre saisie !");

    if search_book(&title) {
        println!("Livre trouvé !");
        println!("{:?}", get_book(title));
    } else {
        println!("Livre non trouvé !");
    }
}

// Détermine si un livre existe
fn search_book(title: &String) -> bool {
    for book in read_library() {
        if title.to_string() == book.title {
            return true;
        }
    }

    false
}

// récupère les informations d'un livre
fn get_book(title: String) -> Book {
    let mut book: Book = Book{
        title: "".to_string(),
        author: "".to_string(),
        isbn: "".to_string(),
        published_year: "".to_string(),
    };

    for record in read_library() {
        if title == record.title {
            book = Book{
                title: record.title,
                author: record.author,
                isbn: record.isbn,
                published_year: record.published_year
            };
            break;
        }
    }

    book
}

// Lecture de la bibliothèque
fn read_library_action() {
    println!("Reading library ...");
    for book in read_library() {
        println!("{:?}", book);
    }
}

fn read_library() -> Vec<Book> {
    let file = File::open(LIBRARY_FILE);
    let mut reader = csv::Reader::from_reader(file);
    let mut vec_records = Vec::<Book>::new();

    for result in reader.deserialize() {
        println!("{:?}", result);
        let record: (String, Vec<String>) = result.unwrap();

        println!("{} , {:?}", record.0, record.1);
        let book: Book = Book {
            title: record.1[0].clone(),
            author: record.1[1].clone(),
            isbn: record.1[2].clone(),
            published_year: record.1[3].clone()
        };
    }

    vec_records
}

// Retire un livre de la librairie
fn remove_book_action() {
    println!("Quel livre voulez vous retirer ?");

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Impossible de lire votre saisie !");

    let mut new_books = Vec::<Book>::new();

    for book in read_library() {
        if title != book.title {
            new_books.push(book);
        }
    }

    write_library(new_books);
}

// Réécriture de la librairie
fn write_library(books: Vec<Book>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(LIBRARY_FILE);

    for book in books {
        &writer.unwrap().write_record(&[book.title, book.author, book.isbn, book.published_year]).expect("TODO: panic message");
    }

    writer.unwrap().flush()?;

    Ok(())
}