
#[derive(Clone, Debug)]
pub struct Music {
    /// 名称
    pub name: String,
    /// 作者
    pub author: String,
    /// 类型
    pub music_type: MusicType,
    /// wyy_id
    pub wid: u64,
    /// qq_id
    pub qid: String,
    /// url
    pub url: String,
    /// 本地文件路径
    pub file_path: String,
}

#[derive(Clone, Debug)]
pub enum MusicType {
    WYY,
    QQ,
    LOCAL
}
