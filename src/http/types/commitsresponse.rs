use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitsResponse{
   skills: Vec<SkillObject>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SkillObject{
    skill_name:String,
    mastery_score:u64,
    subskills: Vec<SubSkill>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SubSkill{
    subskill_name:String,
    mastery_score:u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FirstCommitsResponse{
    pub technologies: Vec<String>
}