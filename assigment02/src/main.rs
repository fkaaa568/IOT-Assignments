#[derive(Debug)]
#[warn(dead_code)]
struct Book{
        Name:String,
        Author:String,
}

impl Book {
    fn book(Name:String,Author:String)-> Book{
        Book{
            Name,
            Author,
        }
    }
}

pub trait Bookinformation {
    fn info(&self)-> String;
}

 impl Bookinformation for Book {
            fn info(&self)-> String{
                format!("The name of Book is :{} And The book Author Name is {}",self.Name,self.Author)
            }
}


fn main() {
        let Book_01 = Book::book(
            "War And Peace".to_string(),
            "Leo Tolstoy".to_string(),
        );
        println!("{:#?}",Book_01);

        let Book_02 = Book{
            Name:"In Search of Lost Time".to_string(),
            Author:"Marcel Proust".to_string(),
        };
        
        println!("{:#?}",Book_02.info());
}
