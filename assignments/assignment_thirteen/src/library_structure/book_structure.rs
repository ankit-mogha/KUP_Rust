use std::collections::HashMap;
#[derive(Debug, PartialEq)]

pub struct Book {
    pub accession_number: i32,
    pub name_of_author: String,
    pub book_title: String,
    pub flag: i8,
}

impl Book {
    /// display method displays all books in Books structure.
    ///
    /// #Arguments
    ///
    /// &self :takes the type of calling object.
    ///
    /// #Return
    ///
    /// Returns Self type of calling object.
    pub fn display(&self) -> Self {
        log::debug!("Accession Number : {}", self.accession_number);
        log::debug!("Author Name : {}", self.name_of_author);
        log::debug!("Book Name : {}", self.book_title);
        if self.flag == 1 {
            log::debug!("Book Status : Available");
        } else {
            log::debug!("Book Status : Issued");
        }
        Book {
            accession_number: self.accession_number,
            name_of_author: (*self.name_of_author).to_owned(),
            book_title: (*self.book_title).to_owned(),
            flag: self.flag,
        }
    }
    /// total_number_of_books method displays all books in Books structure.
    ///
    /// #Arguments
    ///
    /// &self :takes the type of calling object.
    ///
    /// no_books: list of book accession number.
    ///
    /// #Return
    ///
    /// Returns Number of books present in library.
    pub fn total_number_of_books(&self, no_books: &[i32]) -> Option<usize> {
        log::debug!("Total number of books : {}", no_books.len());
        let length = no_books.len();
        if length == 0 {
            None
        } else {
            Some(length)
        }
    }
    /// total_number_of_books method displays all books in Books structure.
    ///
    /// #Arguments
    ///
    /// author : name of author to be search .
    ///
    /// no_books: list of book accession number.
    ///
    /// #Return
    ///
    /// Returns Title of book written by the author.
    pub fn display_books_given_author(
        author: String,
        map_books_authors: &HashMap<String, String>,
    ) -> Option<String> {
        let author_name = author;
        let book_name = map_books_authors.get(&author_name);
        let mut result = String::new();
        match book_name {
            Some(value) => {
                log::debug!("Book name : {}", value);
                result.push_str(value);
            }
            None => {
                result.push_str("author not found");
                log::error!("author not found");
            }
        }
        Some(result)
    }
    /// add_book method adds a new book to library.
    ///
    /// #Arguments
    ///
    /// &self :takes the type of calling object.
    ///
    /// map_book_authors : Hashmap contains authors and there book titles.
    ///
    /// map_number_flag : Hashmap contains book accession number and flag value .
    ///
    /// #Return
    ///
    /// Returns a Confirmation message.
    pub fn add_book(
        &mut self,
        no_books: &mut Vec<i32>,
        map_books_authors: &mut HashMap<String, String>,
        map_number_flag: &mut HashMap<i32, i8>,
    ) -> Option<String> {
        let mut result = String::new();
        if no_books.contains(&self.accession_number) {
            log::error!("Book already in library");
            result.push_str("Book already in library");
        } else {
            {
                String::from(&self.name_of_author);
                String::from(&self.book_title);
                self.flag + 1
            };
            map_books_authors.insert(self.name_of_author.clone(), self.book_title.clone());
            map_number_flag.insert(self.accession_number, self.flag + 1);
            no_books.push(self.accession_number);
            result.push_str("Book Added");
        }
        Some(result)
    }

    /// issue method issues a books.
    ///
    /// #Arguments
    ///
    /// &self :takes the type of calling object.
    ///
    /// #Return
    ///
    /// Returns a Confirmation message.
    pub fn issue(&mut self /*number_flag:&mut HashMap<i32,i8>*/) -> Option<String> {
        let mut result = String::new();
        match self.flag {
            1 => {
                self.flag -= 1;
                log::debug!("Book issued");
                result.push_str("Book issued");
            }
            _ => {
                log::error!("Book already issued to someone");
                result.push_str("Book already issued to someone");
            }
        }
        Some(result)
    }
    /// add_book method adds a new book to library.
    ///
    /// #Arguments
    ///
    /// title : title of the book to be searched.
    ///
    /// books : Hashmap contains authors and there book titles.
    ///
    /// #Return
    ///
    /// Returns number of books with same title.
    pub fn display_number_of_books_of_particular_title(
        title: String,
        books: &HashMap<String, String>,
        count: &mut i8,
    ) -> Option<i8> {
        let mut books_set: Vec<String> = Vec::new();
        for books_names in books.values() {
            books_set.push(books_names.to_string());
        }
        let mut index = 0;
        while index < books_set.len() {
            if books_set[index].eq(&title) {
                *count += 1;
            }
            index += 1;
        }
        log::debug!("Number of books of title {:?} is {}", &title, count);
        if *count == 0 {
            None
        } else {
            Some(*count)
        }
    }
}
