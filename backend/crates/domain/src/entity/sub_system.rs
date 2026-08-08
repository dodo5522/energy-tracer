/// サブシステムエンティティ
///
/// サブシステムは発電設備の構成要素であり、発電状況を記録する際に必要な情報
///
/// # Example
/// * Battery: 蓄電池
/// * Array: 太陽光パネル
/// * WindTurbine: 風力タービン
/// * Controller: 充放電コントローラ
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubSystemEntity {
    /// サブシステム
    pub system: String,
    /// 補足
    pub remark: String,
}
