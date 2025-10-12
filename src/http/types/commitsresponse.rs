use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommitsResponse{
   commit_id:String,
   commit_message:String,
   skills: Vec<SkillObject>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillObject{
    skill_name:String,
    mastery_score:u64,
    justification:String,
    subskills: Vec<SubSkill>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubSkill{
    subskill_name:String,
    mastery_score:u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FirstCommitsResponse{
    pub commit_id:String,
    pub commit_message:String,
    pub technologies: Vec<String>
}