use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct Task
{
    pub name: String,
    #[serde(default="def_str")]
    pub description: String,
    #[serde(default="def_dirs")]
    pub source_dir: PathBuf,
    #[serde(default="def_dirs")]
    pub target_dir: PathBuf,
    #[serde(default="def_timer")]
    pub timer: u64,
    #[serde(default="is_default")]
    pub delete_after_copy: bool,
    //#[serde(default="def_copy_mod")]
    //#[serde(deserialize_with="deserialize_copy_modifier")]
    pub copy_modifier: CopyModifier,
    #[serde(default="is_default")]
    pub is_active: bool,
    ///Типы пакетов которые будут очищаться
    #[serde(default="empty_doc_types")]
    pub clean_types: Vec<String>,
    #[serde(default="is_default")]
    pub generate_exclude_file: bool,
    #[serde(default="def_col")]
    pub color: String,
    pub filters: Filter
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct Filter
{
    #[serde(default="empty_doc_types")]
    pub document_types: Vec<String>,
    #[serde(default="empty_doc_types")]
    pub document_uids: Vec<String>
}

fn is_default() -> bool
{
    false
}
fn def_timer() -> u64
{
    200000
}
fn empty_doc_types() -> Vec<String>
{
    Vec::with_capacity(0)
}
fn def_dirs() -> PathBuf
{
    PathBuf::from("---")
}
fn def_str() -> String
{
    "".to_owned()
}
fn def_col() -> String
{
    "#4f46".to_owned()
}

impl Default for Task
{
    fn default() -> Self 
    {
        Task
        {
            source_dir: PathBuf::from("in"),
            target_dir: PathBuf::from("out"),
            timer: 20000,
            name: "default_task".to_owned(),
            description: "".to_owned(),
            copy_modifier: CopyModifier::CopyAll,
            delete_after_copy: false,
            is_active: false,
            clean_types: vec![],
            generate_exclude_file: false,
            color: def_col(),
            filters: Filter
            {
                document_types: vec![],
                document_uids: vec![]
            }
            
        }
    }
}



#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CopyModifier
{
    CopyAll,
    CopyOnly,
    CopyExcept
}
impl Display for CopyModifier
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", match self 
        {
            CopyModifier::CopyAll => "CopyAll",
            CopyModifier::CopyOnly => "CopyOnly",
            CopyModifier::CopyExcept => "CopyExcept"
        })
    }
}