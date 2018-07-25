#[derive(Serialize, Deserialize, Debug)]
pub struct Task {    
    pub id: Option<String>, // The task id.
    pub name: Option<String>, // The task name.
    pub assignee: Option<String>, // The assignee's id.
    pub owner: Option<String>, // The owner's id.
    pub created: Option<String>, // The date the task was created on. Has the format yyyy-MM-dd'T'HH:mm:ss.
    pub due: Option<String>, // The task's due date. Has the format yyyy-MM-dd'T'HH:mm:ss.
    pub followUp: Option<String>, // The follow-up date for the task. Format yyyy-MM-dd'T'HH:mm:ss.
    pub delegationState: Option<String>, // The task's delegation state. Possible values are PENDING and RESOLVED.
    pub description: Option<String>, // The task's description.
    pub executionId: Option<String>, // The id of the execution the task belongs to.
    pub parentTaskId: Option<String>, // The id the parent task, if this task is a subtask.
    pub priority: Option<i64>, // The task's priority.
    pub processDefinitionId: Option<String>, // The id of the process definition the task belongs to.
    pub processInstanceId: Option<String>, // The id of the process instance the task belongs to.
    pub caseExecutionId: Option<String>, // The id of the case execution the task belongs to.
    pub caseDefinitionId: Option<String>, // The id of the case definition the task belongs to.
    pub caseInstanceId: Option<String>, // The id of the case instance the task belongs to.
    pub taskDefinitionKey: Option<String>, // The task's key.
    pub formKey: Option<String>, // If not null, the form key for the task.
    pub tenantId: Option<String> //	If not null, the tenant id of the task.
}
