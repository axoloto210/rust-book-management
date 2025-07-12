use ::serde::{Deserialize, Serialize};
use kernel::model::book::{Book, event::CreateBook};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

// api向けの型はレイヤードアーキテクチャ採用のためkernel以下で使用できない。
// そのため、kernelで使用できるようkernelのCreateBookに変換する必要がある。そのためにFromを作成する。
impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}
