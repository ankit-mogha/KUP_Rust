#[cfg(test)]
mod tests {
    use crate::library_structure::book_structure::Book;
    use std::collections::HashMap;

    #[test]
    fn check_display_book_info() {
        env_logger::init();
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };
        assert_eq!(book1.display(), book1);
    }

    #[test]
    fn check_total_number_book() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let list_of_books = vec![book1.accession_number, book2.accession_number];
        assert_eq!(book1.total_number_of_books(&list_of_books), 2);
    }

    #[test]
    fn check_display_book_by_author_name() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let mut map_books_authors = HashMap::new();
        map_books_authors.insert(book1.name_of_author.clone(), book1.book_title.clone());
        map_books_authors.insert(book2.name_of_author.clone(), book2.book_title.clone());

        assert_eq!(
            Book::display_books_given_author("Darcey Bell".to_string(), &map_books_authors),
            "A Simple Favor"
        );
    }

    #[test]
    fn check_author_name_not_found() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let mut map_books_authors = HashMap::new();
        map_books_authors.insert(book1.name_of_author.clone(), book1.book_title.clone());
        map_books_authors.insert(book2.name_of_author.clone(), book2.book_title.clone());

        assert_eq!(
            Book::display_books_given_author("Ankit Mogha".to_string(), &map_books_authors),
            "author not found"
        );
    }

    #[test]
    fn check_add_book_in_library() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let mut book3 = Book {
            accession_number: 103,
            name_of_author: String::from("Zoje Stage"),
            book_title: String::from("Baby Teeth"),
            flag: 0,
        };

        let mut list_of_books = vec![book1.accession_number, book2.accession_number];

        let mut map_books_authors = HashMap::new();
        map_books_authors.insert(book1.name_of_author.clone(), book1.book_title.clone());
        map_books_authors.insert(book2.name_of_author.clone(), book2.book_title.clone());

        let mut map_number_flag = HashMap::new();
        map_number_flag.insert(book1.accession_number, book1.flag);
        map_number_flag.insert(book2.accession_number, book2.flag);

        assert_eq!(
            book3.add_book(
                &mut list_of_books,
                &mut map_books_authors,
                &mut map_number_flag
            ),
            "Book Added"
        );
    }

    #[test]
    fn book_already_in_library() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let mut book3 = Book {
            accession_number: 101,
            name_of_author: String::from("Zoje Stage"),
            book_title: String::from("Baby Teeth"),
            flag: 0,
        };

        let mut list_of_books = vec![book1.accession_number, book2.accession_number];

        let mut map_books_authors = HashMap::new();
        map_books_authors.insert(book1.name_of_author.clone(), book1.book_title.clone());
        map_books_authors.insert(book2.name_of_author.clone(), book2.book_title.clone());

        let mut map_number_flag = HashMap::new();
        map_number_flag.insert(book1.accession_number, book1.flag);
        map_number_flag.insert(book2.accession_number, book2.flag);

        assert_eq!(
            book3.add_book(
                &mut list_of_books,
                &mut map_books_authors,
                &mut map_number_flag
            ),
            "Book already in library"
        );
    }

    #[test]
    fn check_issue_book() {
        let mut book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        assert_eq!(book1.issue(), "Book issued");
    }

    #[test]
    fn check_when_book_is_already_issued() {
        let mut book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };
        book1.issue();
        assert_eq!(book1.issue(), "Book already issued to someone");
    }

    #[test]
    fn check_number_of_books_of_same_title() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let book3 = Book {
            accession_number: 101,
            name_of_author: String::from("Zoje Stage"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let mut map_books_authors = HashMap::new();
        map_books_authors.insert(book1.name_of_author.clone(), book1.book_title.clone());
        map_books_authors.insert(book2.name_of_author.clone(), book2.book_title.clone());
        map_books_authors.insert(book3.name_of_author.clone(), book3.book_title.clone());

        let mut count: i8 = 0;
        assert_eq!(
            Book::display_number_of_books_of_particular_title(
                "And Then There Were None".to_string(),
                &map_books_authors,
                &mut count
            ),
            [2]
        );
    }

    #[test]
    fn book_title_not_found() {
        let book1 = Book {
            accession_number: 101,
            name_of_author: String::from("Darcey Bell"),
            book_title: String::from("A Simple Favor"),
            flag: 1,
        };

        let book2 = Book {
            accession_number: 102,
            name_of_author: String::from("Agatha Christie"),
            book_title: String::from("And Then There Were None"),
            flag: 1,
        };

        let book3 = Book {
            accession_number: 101,
            name_of_author: String::from("Zoje Stage"),
            book_title: String::from("Baby Teeth"),
            flag: 1,
        };

        let mut map_books_authors = HashMap::new();
        map_books_authors.insert(book1.name_of_author.clone(), book1.book_title.clone());
        map_books_authors.insert(book2.name_of_author.clone(), book2.book_title.clone());
        map_books_authors.insert(book3.name_of_author.clone(), book3.book_title.clone());

        let mut count: i8 = 0;
        assert_eq!(
            Book::display_number_of_books_of_particular_title(
                "Harry Potter".to_string(),
                &map_books_authors,
                &mut count
            ),
            [0]
        );
    }
}
