use super::stages::finished::Finished;

pub enum Advancement<NextStage> {
    NextStage(NextStage),
    Finished(Finished),
}
