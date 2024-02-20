use crate::{metadata::metadatatype, prelude::*, utils::string_to_datetime};
use chrono::{DateTime, Utc};
use metadatatype::MetaDataType;
use serde_derive::{Deserialize, Serialize};

/// Striped down version of Metadata
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Dataset {
    pub pangaea_id: Option<String>,
    pub title: String,
    pub text_abstract: Option<String>,
    pub authors: Vec<Author>,
    pub publication_date: Option<DateTime<Utc>>,
    pub uri: Option<String>,
    pub extent: Option<Extent>,
    pub keywords: Vec<String>,
    pub license: Option<License>,
    pub projects: Vec<Project>,
}

fn get_pangaea_id(md: &MetaDataType) -> Option<String> {
    match &md.citation.citation_type.id_attributes.id {
        None => None,
        Some(id) => id.strip_prefix("dataset").map(|s| s.to_owned()),
    }
}

fn get_title(md: &MetaDataType) -> String {
    md.citation.citation_type.title.to_owned()
}

fn get_abstract(md: &MetaDataType) -> Option<String> {
    md.text_abstract.to_owned()
}

fn get_authors(md: &MetaDataType) -> Vec<Author> {
    md.citation
        .citation_type
        .authors
        .clone()
        .into_iter()
        .map(|a| a.into())
        .collect()
}

fn get_publication_date(md: &MetaDataType) -> Option<DateTime<Utc>> {
    match &md.citation.date_time.clone().map(string_to_datetime) {
        None => None,
        Some(v) => v.to_owned(),
    }
}

fn get_uri(md: &MetaDataType) -> Option<String> {
    md.citation.citation_type.uri.to_owned()
}

fn get_extent(md: &MetaDataType) -> Option<Extent> {
    md.extent.clone().map(|ext| ext.into())
}

fn get_keywords(md: &MetaDataType) -> Vec<String> {
    md.keywords
        .clone()
        .map(|k| k.keywords)
        .unwrap_or_default()
        .into_iter()
        .map(|kt| kt.text)
        .collect()
}

fn get_license(md: &MetaDataType) -> Option<License> {
    match md.license.clone() {
        None => None,
        Some(license) => License::try_from(license).ok(),
    }
}

fn get_projects(md: &MetaDataType) -> Vec<Project> {
    md.projects
        .clone()
        .into_iter()
        .filter_map(|p| Project::try_from(p).ok())
        .collect()
}

impl From<metadatatype::MetaDataType> for Dataset {
    fn from(md: metadatatype::MetaDataType) -> Self {
        Dataset {
            pangaea_id: get_pangaea_id(&md),
            title: get_title(&md),
            text_abstract: get_abstract(&md),
            authors: get_authors(&md),
            publication_date: get_publication_date(&md),
            uri: get_uri(&md),
            extent: get_extent(&md),
            keywords: get_keywords(&md),
            license: get_license(&md),
            projects: get_projects(&md),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Author {
    pub last_name: String,
    pub first_name: Option<String>,
    pub e_mail: Option<String>,
    pub uri: Option<String>,
    pub orcid: Option<String>,
    pub affiliations: Vec<Institution>,
}

impl From<metadatatype::ResponsiblePartyType> for Author {
    fn from(party: metadatatype::ResponsiblePartyType) -> Self {
        Self {
            last_name: party.last_name,
            first_name: party.first_name,
            e_mail: party.e_mail,
            uri: party.uri,
            orcid: party.orcid,
            affiliations: party
                .affiliations
                .into_iter()
                .map(|aff| aff.into())
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Institution {
    pub name: String,
    pub uri: Option<String>,
    pub ror: Option<String>,
    pub crossref_funder_id: Option<String>,
}

impl From<metadatatype::InstitutionType> for Institution {
    fn from(inst: metadatatype::InstitutionType) -> Self {
        Self {
            name: inst.linked_name_type.name,
            uri: inst.linked_name_type.uri,
            ror: inst.ror,
            crossref_funder_id: inst.crossref_funder_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Extent {
    pub geographic: Option<Geographic>,
    pub temporal: Option<Temporal>,
    pub elevation: Option<Elevation>,
}

impl From<metadatatype::ExtentType> for Extent {
    fn from(extent: metadatatype::ExtentType) -> Self {
        Self {
            geographic: extent.geographic.map(|g| g.into()),
            temporal: extent.temporal.map(|t| t.into()),
            elevation: extent.elevation.map(|e| e.into()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Geographic {
    pub west_bound_longitude: f64,
    pub east_bound_longitude: f64,
    pub south_bound_latitude: f64,
    pub north_bound_latitude: f64,
    pub mean_longitude: f64,
    pub mean_latitude: f64,
}

impl From<metadatatype::Geographic> for Geographic {
    fn from(geo: metadatatype::Geographic) -> Self {
        Self {
            west_bound_longitude: geo.west_bound_longitude,
            east_bound_longitude: geo.east_bound_longitude,
            south_bound_latitude: geo.south_bound_latitude,
            north_bound_latitude: geo.north_bound_latitude,
            mean_longitude: geo.mean_longitude,
            mean_latitude: geo.mean_latitude,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Temporal {
    pub min_date_time: Option<DateTime<Utc>>,
    pub max_date_time: Option<DateTime<Utc>>,
}

impl From<metadatatype::Temporal> for Temporal {
    fn from(temp: metadatatype::Temporal) -> Self {
        let min_date_time: Option<DateTime<Utc>> = string_to_datetime(temp.min_date_time);
        let max_date_time: Option<DateTime<Utc>> = string_to_datetime(temp.max_date_time);

        Self {
            min_date_time,
            max_date_time,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Elevation {
    pub name: String,
    pub unit: Option<String>,
    pub min: f64,
    pub max: f64,
}

impl From<metadatatype::Elevation> for Elevation {
    fn from(el: metadatatype::Elevation) -> Self {
        Self {
            name: el.name,
            unit: el.unit,
            min: el.min,
            max: el.max,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct License {
    pub label: String,
    pub name: String,
    pub uri: String,
}

impl TryFrom<metadatatype::LinkedLabelNameType> for License {
    type Error = Error;
    fn try_from(md_license: metadatatype::LinkedLabelNameType) -> Result<Self> {
        let label = md_license
            .label
            .ok_or(Error::Generic("License label is None".to_string()))?;
        let name = md_license
            .name
            .ok_or(Error::Generic("License name is None".to_string()))?;
        let uri = md_license
            .uri
            .ok_or(Error::Generic("License uri is None".to_string()))?;

        Ok(Self { label, name, uri })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Project {
    pub project_type: String,
    pub institution: Institution,
    pub label: String,
    pub name: String,
    pub uri: String,
}

impl TryFrom<metadatatype::ProjectType> for Project {
    type Error = Error;
    fn try_from(project: metadatatype::ProjectType) -> Result<Self> {
        let project_type = project
            .project_type
            .ok_or(Error::Generic("Project project_type is None".to_string()))?;
        let institution = project
            .institution
            .ok_or(Error::Generic("Project institution is None".to_string()))?;
        let label = project
            .linked_label_name_type
            .label
            .ok_or(Error::Generic("Project label is None".to_string()))?;
        let name = project
            .linked_label_name_type
            .name
            .ok_or(Error::Generic("Project name is None".to_string()))?;
        let uri = project
            .linked_label_name_type
            .uri
            .ok_or(Error::Generic("Project uri is None".to_string()))?;

        Ok(Self {
            project_type,
            institution: institution.into(),
            label,
            name,
            uri,
        })
    }
}
