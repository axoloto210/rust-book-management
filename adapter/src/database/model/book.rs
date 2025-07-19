use kernel::model::book::Book;
use kernel::model::id::BookId;

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

// kernelで定義したBook構造体に合わせるため、Fromを実装。Fromを実装すると同時にIntoも実装（ブランケット実装）され、型変換がしやすくなる。
// ブランケット実装は特定の条件を満たすすべての型に対して、一括でトレイトを実装する仕組みで、以下のようにintoが実装される。
// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
