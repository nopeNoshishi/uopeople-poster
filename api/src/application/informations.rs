//! アプリケーション InforamationのCRUD処理
//!
//! インフラ層で定義したリポジトリの実態とメソッドで、
//! ドメインオブジェクトを使ってロジックを作成しています。

use crate::domains::repository::informations::*;
use crate::infrastructures::repositoryimpl::informations::InformationRepositoryImpl;

use anyhow::Result;

pub fn create(info: &Information) -> Result<()> {
    let info_repo = InformationRepositoryImpl {
        database: "postgres".to_string(),
    };

    info_repo.create(info)
}

pub fn read_all() -> Result<Vec<Information>> {
    let info_repo = InformationRepositoryImpl {
        database: "postgres".to_string(),
    };

    info_repo.read_all()
}

pub fn update(info: &Information) -> Result<()> {
    let info_repo = InformationRepositoryImpl {
        database: "postgres".to_string(),
    };

    info_repo.update(info)
}

pub fn delete(info_id: &InformationId) -> Result<()> {
    let info_repo = InformationRepositoryImpl {
        database: "postgres".to_string(),
    };

    info_repo.delete(info_id)
}
