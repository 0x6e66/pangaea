//! Generated with [xgen](https://github.com/xuri/xgen) from the [PANGAEA metadata scheme](https://ws.pangaea.de/schemas/pangaea/MetaData.xsd) and modified afterwards

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct SizeType {
    #[yaserde(attribute)]
    pub unit: Option<String>,
    #[yaserde(text)]
    pub text: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct MetaDataType {
    #[yaserde(attribute)]
    pub version: String,
    #[yaserde(rename = "citation", prefix = "md")]
    pub citation: DataSetCitationType,
    #[yaserde(rename = "abstract", prefix = "md")]
    pub text_abstract: Option<String>,
    #[yaserde(rename = "reference", prefix = "md")]
    pub references: Vec<ReferenceType>,
    #[yaserde(rename = "extent", prefix = "md")]
    pub extent: Option<ExtentType>,
    #[yaserde(rename = "project", prefix = "md")]
    pub projects: Vec<ProjectType>,
    #[yaserde(rename = "award", prefix = "md")]
    pub awards: Vec<AwardType>,
    #[yaserde(rename = "event", prefix = "md")]
    pub events: Vec<EventType>,
    #[yaserde(rename = "attribute", prefix = "md")]
    pub attributes: Vec<AttributeType>,
    #[yaserde(rename = "size", prefix = "md")]
    pub size: SizeType,
    #[yaserde(rename = "license", prefix = "md")]
    pub license: Option<LinkedLabelNameType>,
    #[yaserde(rename = "comment", prefix = "md")]
    pub comment: Option<String>,
    #[yaserde(rename = "matrixColumn", prefix = "md")]
    pub matrix_columns: Vec<ColumnType>,
    #[yaserde(rename = "status", prefix = "md")]
    pub status: Option<StatusType>,
    #[yaserde(rename = "additionalSearchableContent", prefix = "md")]
    pub additional_searchable_content: Option<String>,
    #[yaserde(rename = "keywords", prefix = "md")]
    pub keywords: Option<KeywordsType>,
    #[yaserde(rename = "history", prefix = "md")]
    pub history: Vec<HistoryType>,
    #[yaserde(rename = "technicalInfo", prefix = "md")]
    pub technical_info: TechnicalInfoType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct DataSetCitationType {
    #[yaserde(attribute, rename = "authorsAreEditors")]
    pub authors_are_editors: bool,
    #[yaserde(rename = "alternativeTitle", prefix = "md")]
    pub alternative_title: Option<String>,
    #[yaserde(rename = "dateTime", prefix = "md")]
    pub date_time: Option<String>,
    #[yaserde(rename = "parentURI", prefix = "md")]
    pub parent_uri: Option<String>,
    #[yaserde(rename = "supplementTo", prefix = "md")]
    pub supplement_to: Option<ReferenceType>,
    #[yaserde(flatten)]
    pub citation_type: CitationType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct Source {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    pub type_attr: Option<String>,
    #[yaserde(text)]
    pub text: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct CitationAttributes {
    #[yaserde(attribute, rename = "relationTypeId")]
    pub relation_type_id: Option<u32>,
    #[yaserde(attribute, rename = "relationType")]
    pub relation_type: Option<String>,
    #[yaserde(attribute, rename = "dataciteRelType")]
    pub datacite_rel_type: Option<String>,
    #[yaserde(attribute, rename = "group")]
    pub group: Option<String>,
    #[yaserde(attribute, rename = "typeId")]
    pub type_id: Option<String>,
    #[yaserde(attribute, rename = "typeName")]
    pub type_name: Option<String>,
    #[yaserde(attribute, rename = "comment")]
    pub comment: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct CitationType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(flatten)]
    pub citation_attributes: CitationAttributes,
    #[yaserde(rename = "author", prefix = "md")]
    pub authors: Vec<ResponsiblePartyType>,
    #[yaserde(rename = "year", prefix = "md")]
    pub year: Option<u32>,
    #[yaserde(rename = "prepubStatus", prefix = "md")]
    pub prepub_status: Option<String>,
    #[yaserde(rename = "title", prefix = "md")]
    pub title: String,
    #[yaserde(rename = "source", prefix = "md")]
    pub sources: Vec<Source>,
    #[yaserde(rename = "volume", prefix = "md")]
    pub volume: Option<String>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct Geographic {
    #[yaserde(rename = "westBoundLongitude", prefix = "md")]
    pub west_bound_longitude: f64,
    #[yaserde(rename = "eastBoundLongitude", prefix = "md")]
    pub east_bound_longitude: f64,
    #[yaserde(rename = "southBoundLatitude", prefix = "md")]
    pub south_bound_latitude: f64,
    #[yaserde(rename = "northBoundLatitude", prefix = "md")]
    pub north_bound_latitude: f64,
    #[yaserde(rename = "meanLongitude", prefix = "md")]
    pub mean_longitude: f64,
    #[yaserde(rename = "meanLatitude", prefix = "md")]
    pub mean_latitude: f64,
    #[yaserde(rename = "location", prefix = "md")]
    pub location: Option<LinkedNameType>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct Temporal {
    #[yaserde(rename = "minDateTime", prefix = "md")]
    pub min_date_time: String,
    #[yaserde(rename = "maxDateTime", prefix = "md")]
    pub max_date_time: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct Elevation {
    #[yaserde(rename = "name", prefix = "md", attribute)]
    pub name: String,
    #[yaserde(rename = "unit", prefix = "md", attribute)]
    pub unit: Option<String>,
    #[yaserde(rename = "geocodeId", prefix = "md", attribute)]
    pub geocode_id: Option<String>,
    #[yaserde(rename = "min", prefix = "md")]
    pub min: f64,
    #[yaserde(rename = "max", prefix = "md")]
    pub max: f64,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct TopoType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(text)]
    pub text: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ExtentType {
    #[yaserde(rename = "geographic", prefix = "md")]
    pub geographic: Option<Geographic>,
    #[yaserde(rename = "temporal", prefix = "md")]
    pub temporal: Option<Temporal>,
    #[yaserde(rename = "elevation", prefix = "md")]
    pub elevation: Option<Elevation>,
    #[yaserde(rename = "topoType", prefix = "md")]
    pub topo_type: Option<TopoType>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct EventType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "label", prefix = "md")]
    pub label: String,
    #[yaserde(rename = "optionalLabel", prefix = "md")]
    pub optional_label: Option<String>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
    #[yaserde(rename = "latitude", prefix = "md")]
    pub latitude: Option<f64>,
    #[yaserde(rename = "longitude", prefix = "md")]
    pub longitude: Option<f64>,
    #[yaserde(rename = "elevation", prefix = "md")]
    pub elevation: Option<f64>,
    #[yaserde(rename = "dateTime", prefix = "md")]
    pub date_time: Option<String>,
    #[yaserde(rename = "latitude2", prefix = "md")]
    pub latitude2: Option<f64>,
    #[yaserde(rename = "longitude2", prefix = "md")]
    pub longitude2: Option<f64>,
    #[yaserde(rename = "elevation2", prefix = "md")]
    pub elevation2: Option<f64>,
    #[yaserde(rename = "dateTime2", prefix = "md")]
    pub date_time2: Option<String>,
    #[yaserde(rename = "attribute", prefix = "md")]
    pub attributes: Vec<AttributeType>,
    #[yaserde(rename = "location", prefix = "md")]
    pub location: Option<LinkedNameType>,
    #[yaserde(rename = "campaign", prefix = "md")]
    pub campaign: Option<CampaignType>,
    #[yaserde(rename = "basis", prefix = "md")]
    pub basis: Option<BasisType>,
    #[yaserde(rename = "method", prefix = "md")]
    pub method: Option<MethodType>,
    #[yaserde(rename = "comment", prefix = "md")]
    pub comment: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct LinkedLabelNameType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "label", prefix = "md")]
    pub label: Option<String>,
    #[yaserde(rename = "name", prefix = "md")]
    pub name: Option<String>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ProjectType {
    #[yaserde(rename = "type", prefix = "md", attribute)]
    pub project_type: Option<String>,
    #[yaserde(rename = "institution", prefix = "md")]
    pub institution: Option<InstitutionType>,
    #[yaserde(flatten)]
    pub linked_label_name_type: LinkedLabelNameType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct AwardType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "funder", prefix = "md")]
    pub funder: InstitutionType,
    #[yaserde(rename = "number", prefix = "md")]
    pub number: String,
    #[yaserde(rename = "title", prefix = "md")]
    pub title: Option<String>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ReferenceAttributes {
    #[yaserde(attribute, rename = "dataciteRelType")]
    pub datacite_rel_type: Option<String>,
    #[yaserde(attribute, rename = "group")]
    pub group: Option<String>,
    #[yaserde(attribute, rename = "id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "relationType")]
    pub relation_type: Option<String>,
    #[yaserde(attribute, rename = "relationTypeId")]
    pub relation_type_id: Option<String>,
    #[yaserde(attribute, rename = "typeId")]
    pub type_id: Option<String>,
    #[yaserde(attribute, rename = "typeName")]
    pub type_name: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ReferenceType {
    #[yaserde(prefix = "md")]
    pub pages: Option<String>,
    #[yaserde(rename = "attachmentURI", prefix = "md")]
    pub attachment_uri: Vec<String>,
    #[yaserde(flatten)]
    pub citation: CitationType,
    #[yaserde(flatten)]
    pub reference_attributes: ReferenceAttributes,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct IdAttributes {
    #[yaserde(attribute, rename = "id")]
    pub id: Option<String>,
    #[yaserde(attribute, rename = "semanticURI")]
    pub semantic_uri: Option<String>,
    #[yaserde(attribute, rename = "relatedTermIds")]
    pub related_term_ids: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ResponsiblePartyType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "lastName", prefix = "md")]
    pub last_name: String,
    #[yaserde(rename = "firstName", prefix = "md")]
    pub first_name: Option<String>,
    #[yaserde(rename = "eMail", prefix = "md")]
    pub e_mail: Option<String>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
    #[yaserde(rename = "orcid", prefix = "md")]
    pub orcid: Option<String>,
    #[yaserde(rename = "affiliation", prefix = "md")]
    pub affiliations: Vec<InstitutionType>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct CampaignType {
    #[yaserde(rename = "chiefScientist", prefix = "md")]
    pub chief_scientists: Vec<String>,
    #[yaserde(rename = "start", prefix = "md")]
    pub start: Option<String>,
    #[yaserde(rename = "end", prefix = "md")]
    pub end: Option<String>,
    #[yaserde(rename = "attribute", prefix = "md")]
    pub attribute: Vec<AttributeType>,
    #[yaserde(flatten)]
    pub linked_name_type: LinkedNameType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct BasisType {
    #[yaserde(rename = "callSign", prefix = "md")]
    pub call_sign: Option<String>,
    #[yaserde(rename = "IMOnumber", prefix = "md")]
    pub imo_number: Option<String>,
    #[yaserde(flatten)]
    pub linked_name_type: LinkedNameType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct StatusType {
    #[yaserde(rename = "curationLevel", prefix = "md")]
    pub curation_level: Vec<LinkedLabelNameType>,
    #[yaserde(rename = "processingLevel", prefix = "md")]
    pub processing_level: Vec<LinkedLabelNameType>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct KeywordsType {
    #[yaserde(rename = "techKeyword", prefix = "md")]
    pub tech_keywords: Vec<KeywordType>,
    #[yaserde(rename = "keyword", prefix = "md")]
    pub keywords: Vec<KeywordType>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct KeywordType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "type", prefix = "md", attribute)]
    pub keyword_type: KeywordTypeType,
    #[yaserde(text)]
    pub text: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub enum KeywordTypeType {
    #[yaserde(rename = "fromDatabase", prefix = "md")]
    FromDatabase,
    #[default]
    #[yaserde(rename = "autoGenerated", prefix = "md")]
    AutoGenerated,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct Entry {
    #[yaserde(rename = "key", attribute)]
    pub key: String,
    #[yaserde(rename = "value", attribute)]
    pub value: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct TechnicalInfoType {
    #[yaserde(rename = "entry")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ColumnType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "col", attribute)]
    pub col: i32,
    #[yaserde(rename = "type", attribute)]
    pub columntypetype: ColumnTypeType,
    #[yaserde(rename = "source", attribute)]
    pub source: ColumnSourceType,
    #[yaserde(rename = "format", attribute)]
    pub format: Option<String>,
    #[yaserde(rename = "label", attribute)]
    pub label: Option<String>,
    #[yaserde(rename = "behaviour", attribute)]
    pub behavior: Option<i32>,
    #[yaserde(rename = "parameter", attribute)]
    pub parameter: ParameterType,
    #[yaserde(rename = "method", prefix = "md")]
    pub method: Option<MethodType>,
    #[yaserde(rename = "PI", prefix = "md")]
    pub pi: Option<ResponsiblePartyType>,
    #[yaserde(rename = "comment", prefix = "md")]
    pub comment: Option<String>,
    #[yaserde(rename = "caption", prefix = "md")]
    pub caption: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub enum ColumnTypeType {
    #[yaserde(rename = "numeric", prefix = "md")]
    Numeric,
    #[yaserde(rename = "string", prefix = "md")]
    #[default]
    String,
    #[yaserde(rename = "datetime", prefix = "md")]
    Datetime,
    #[yaserde(rename = "binary", prefix = "md")]
    Binary,
    #[yaserde(rename = "uri", prefix = "md")]
    Uri,
    #[yaserde(rename = "filename", prefix = "md")]
    Filename,
    #[yaserde(rename = "unknown", prefix = "md")]
    Unknown,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub enum ColumnSourceType {
    #[yaserde(rename = "event", prefix = "md")]
    #[default]
    Event,
    #[yaserde(rename = "geocode", prefix = "md")]
    Geocode,
    #[yaserde(rename = "data", prefix = "md")]
    Data,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct GroupType {
    #[yaserde(text)]
    pub text: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct ParameterType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(rename = "name", prefix = "md")]
    pub name: String,
    #[yaserde(rename = "shortName", prefix = "md")]
    pub short_name: String,
    #[yaserde(rename = "unit", prefix = "md")]
    pub unit: Option<String>,
    #[yaserde(rename = "group", prefix = "md")]
    pub group: Option<GroupType>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
    #[yaserde(rename = "term", prefix = "md")]
    pub term: Vec<TermType>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct HistoryType {
    #[yaserde(rename = "datetime", prefix = "md", attribute)]
    pub datetime: String,
    #[yaserde(text)]
    pub text: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct LinkedNameType {
    #[yaserde(flatten)]
    pub id_attributes: IdAttributes,
    #[yaserde(prefix = "md")]
    pub name: String,
    #[yaserde(rename = "optionalName", prefix = "md")]
    pub optional_name: Option<String>,
    #[yaserde(rename = "URI", prefix = "md")]
    pub uri: Option<String>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct TermType {
    #[yaserde(rename = "startOffset", prefix = "md", attribute)]
    pub start_offset: Option<i32>,
    #[yaserde(rename = "endOffset", prefix = "md", attribute)]
    pub end_offset: Option<i32>,
    #[yaserde(rename = "fragment", prefix = "md", attribute)]
    pub fragment: Option<i32>,
    #[yaserde(rename = "terminologyLabel", prefix = "md", attribute)]
    pub terminology_label: String,
    #[yaserde(rename = "terminologyId", prefix = "md", attribute)]
    pub terminology_id: i32,
    #[yaserde(flatten)]
    pub linked_name_type: LinkedNameType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct MethodType {
    #[yaserde(rename = "term", prefix = "md")]
    pub terms: Vec<TermType>,
    #[yaserde(flatten)]
    pub linked_name_type: LinkedNameType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct InstitutionType {
    #[yaserde(rename = "ROR", prefix = "md")]
    pub ror: Option<String>,
    #[yaserde(rename = "crossrefFunderId", prefix = "md")]
    pub crossref_funder_id: Option<String>,
    #[yaserde(flatten)]
    pub linked_name_type: LinkedNameType,
}

#[derive(Debug, YaDeserialize, YaSerialize, Deserialize, Serialize, PartialEq, Default, Clone)]
#[yaserde(prefix = "md", namespace = "md: http://www.pangaea.de/MetaData")]
pub struct AttributeType {
    #[yaserde(rename = "attid", attribute)]
    pub id: i32,
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(text)]
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct ElasticSearchResponse {
    pub took: u32,
    pub timed_out: bool,
    pub _shards: Shards,
    pub hits: Hits,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Shards {
    pub total: u32,
    pub successful: u32,
    pub skipped: u32,
    pub failed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Hits {
    pub total: u32,
    pub max_score: Option<f32>,
    pub hits: Vec<SearchHit>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct SearchHit {
    pub _index: String,
    pub _type: String,
    pub _id: String,
    pub _score: Option<f32>,
    pub _source: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct IndexHit {
    pub _index: String,
    pub _type: String,
    pub _id: String,
    pub _version: i32,
    pub found: bool,
    pub _source: Value,
}
