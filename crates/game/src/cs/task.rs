use std::{ffi, marker::PhantomData};
use windows::core::PCWSTR;

use crate::dl::DLRuntimeClass;
use crate::DLRFLocatable;
use crate::{dl::DLPlainLightMutex, fd4::{FD4BasicHashString, FD4Time}, Tree, Vector};

#[repr(C)]
pub struct FD4TaskBaseVMT  {
    /// Getter for DLRF runtime metadata.
    pub get_runtime_class: fn(*const FD4TaskBase) -> *const DLRuntimeClass,
    /// Destructor
    pub destructor: fn(*const FD4TaskBase),
    /// Gets called by the runtime
    pub execute: fn(*const FD4TaskBase, *const FD4TaskData),
}

#[repr(C)]
pub struct FD4TaskBase {
    pub vftable: *const FD4TaskBaseVMT,
    pub unk8: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct FD4TaskData {
    pub delta_time: FD4Time,
    pub task_group_id: u32,
    pub seed: i32,
}

#[repr(C)]
pub struct CSEzTaskVMT  {
    pub task_base: FD4TaskBaseVMT,
    /// Called by execute() in the case of CSEzTask.
    pub eztask_execute: fn(),
    /// Called to register the task to the appropriate runtime.
    pub register_task: fn(),
    /// Called to free up the task.
    pub free_task: fn(),
    /// Getter for the task group.
    pub get_task_group: fn(),
}

#[repr(C)]
pub struct CSEzTask {
    pub vftable: *const CSEzTaskVMT,
    pub unk8: u32,
    pub _padc: u32,
    pub task_proxy: usize,
}

#[repr(C)]
pub struct CSEzUpdateTask<'a, TSubject> {
    pub base_task: CSEzTask,

    /// Whatever this update task is operating on
    pub subject: &'a TSubject,

    /// Takes in the subject and the delta time
    pub executor: fn(&'a TSubject, f32),
}

#[repr(C)]
pub struct CSTaskGroup<'a> {
    pub vftable: usize,
    pub task_groups: [&'a CSTimeLineTaskGroupIns; 168],
}

impl DLRFLocatable for CSTaskGroup<'_> {
    const DLRF_NAME: &'static str = "CSTaskGroup";
}

#[repr(C)]
pub struct CSTaskGroupIns {
    pub vftable: usize,
    pub name: FD4BasicHashString,
    unk40: [u8; 0x10],
}

#[repr(C)]
pub struct CSTimeLineTaskGroupIns {
    pub base: CSTaskGroupIns,
    pub step_impl: usize,
    unk60: [u8; 0x20],
}

#[repr(C)]
pub struct CSTaskImp<'a> {
    pub vftable: usize,
    pub inner: &'a CSTask<'a>,
}

impl DLRFLocatable for CSTaskImp<'_> {
    const DLRF_NAME: &'static str = "CSTask";
}

#[repr(C)]
pub struct CSTaskBase {
    pub vftable: usize,
    pub allocator1: usize,
    pub task_groups: Vector<TaskGroupEntry>,
    pub task_group_index_max: u32,
    _pad34: u32,
}

#[repr(C)]
pub struct TaskGroupEntry {
    pub index: u32,
    pub name: [u16; 64],
    pub active: bool,
}

#[repr(C)]
pub struct CSTask<'a> {
    pub task_base: CSTaskBase,
    pub allocator2: usize,
    pub unk40: usize,
    pub unk48: [usize; 3],
    pub unk60: [usize; 3],
    pub task_runner_manager: &'a CSTaskRunnerManager<'a>,
    pub task_runners: [&'a CSTaskRunner<'a>; 6],
    pub task_runners_ex: [&'a CSTaskRunnerEx; 6],
    pub unke0: usize,
}

#[repr(C)]
pub struct CSTaskRunner<'a> {
    pub vftable: usize,
    pub task_queue: usize,
    pub task_runner_manager: &'a CSTaskRunnerManager<'a>,
    pub unk18: u32,
    _pad1c: u32,
    pub unk_string: PCWSTR,
}

#[repr(C)]
pub struct CSTaskRunnerEx {
    // TODO
}

#[repr(C)]
pub struct FD4TaskQueue {
    pub vftable: usize,
    pub allocator: usize,
    pub entries_tree: Tree<FD4TaskGroup>,
    pub entries_vector: Vector<FD4TaskGroup>,
}

#[repr(C)]
pub struct FD4TaskGroup {
    pub vftable: usize,
}

#[repr(C)]
pub struct CSTaskRunnerManager<'a> {
    pub allocator: usize,
    pub concurrent_task_group_count: usize,
    pub concurrent_task_group_policy: &'a TaskGroupConcurrency,
    pub current_concurrent_task_group: u32,
    pub unk1c: u32,
    pub unk20: u32,
    _pad24: u32,
    pub mutex: DLPlainLightMutex,
    pub signals: [DLPlainConditionSignal; 6],
    pub unkb8: u32,
    pub unkbc: u32,
    pub unkc0: u32,
    pub unkc4: u32,
    pub unkc8: u32,
    pub unkcc: u32,
    pub unkd0: u32,
    pub unkd4: u32,
}

#[repr(C)]
pub struct FD4TaskRequestEntry {
    pub task: *const FD4TaskBase,
}

#[repr(C)]
pub struct DLPlainConditionSignal {
    pub vftable: usize,
    pub event_handle: usize,
}

#[repr(C)]
pub struct TaskGroupConcurrency {
    pub slots: [TaskGroupConcurrencySlot; 6],
}

#[repr(C)]
pub struct TaskGroupConcurrencySlot {
    pub task_group_index: u32,
    pub task_group_concurrency_type: u32,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum CSTaskGroupIndex {
    FrameBegin,
    SteamThread0,
    SteamThread1,
    SteamThread2,
    SteamThread3,
    SteamThread4,
    SteamThread5,
    SystemStep,
    ResStep,
    PadStep,
    GameFlowStep,
    EndShiftWorldPosition,
    GameMan,
    TaskLineIdx_Sys,
    TaskLineIdx_Test,
    TaskLineIdx_NetworkFlowStep,
    TaskLineIdx_InGame_InGameStep,
    TaskLineIdx_InGame_InGameStayStep,
    MovieStep,
    RemoStep,
    TaskLineIdx_InGame_MoveMapStep,
    FieldArea_EndWorldAiManager,
    EmkSystem_Pre,
    EmkSystem_ConditionStatus,
    EmkSystem_Post,
    EventMan,
    FlverResDelayDelectiionBegin,
    TaskLineIdx_InGame_FieldAreaStep,
    TaskLineIdx_InGame_TestNetStep,
    TaskLineIdx_InGame_InGameMenuStep,
    TaskLineIdx_InGame_TitleMenuStep,
    TaskLineIdx_InGame_CommonMenuStep,
    TaskLineIdx_FrpgNet_Sys,
    TaskLineIdx_FrpgNet_Lobby,
    TaskLineIdx_FrpgNet_ConnectMan,
    TaskLineIdx_FrpgNet_Connect,
    TaskLineIdx_FrpgNet_Other,
    SfxMan,
    FaceGenMan,
    FrpgNetMan,
    NetworkUserManager,
    SessionManager,
    BlockList,
    LuaConsoleServer,
    RmiMan,
    ResMan,
    SfxDebugger,
    REMOTEMAN,
    Geom_WaitActivateFade,
    Geom_UpdateDraw,
    Grass_BatchUpdate,
    Grass_ResourceLoadKick,
    Grass_ResourceLoad,
    Grass_ResourceCleanup,
    WorldChrMan_Respawn,
    WorldChrMan_Prepare,
    ChrIns_CalcUpdateInfo_PerfBegin,
    ChrIns_CalcUpdateInfo,
    ChrIns_CalcUpdateInfo_PerfEnd,
    WorldChrMan_PrePhysics,
    WorldChrMan_CalcOmissionLevel_Begin,
    WorldChrMan_CalcOmissionLevel,
    WorldChrMan_CalcOmissionLevel_End,
    WorldChrMan_ConstructUpdateList,
    WorldChrMan_ChrNetwork,
    ChrIns_Prepare,
    ChrIns_NaviCache,
    ChrIns_AILogic_PerfBegin,
    ChrIns_AILogic,
    ChrIns_AILogic_PerfEnd,
    AI_SimulationStep,
    ChrIns_PreBehavior,
    ChrIns_PreBehaviorSafe,
    GeomModelInsCreatePartway_Begin,
    HavokBehavior,
    GeomModelInsCreatePartway_End,
    ChrIns_BehaviorSafe,
    ChrIns_PrePhysics_Begin,
    ChrIns_PrePhysics,
    ChrIns_PrePhysics_End,
    NetFlushSendData,
    ChrIns_PrePhysicsSafe,
    ChrIns_RagdollSafe,
    ChrIns_GarbageCollection,
    GeomModelInsCreate,
    AiBeginCollectGabage,
    WorldChrMan_Update_RideCheck,
    InGameDebugViewer,
    LocationStep,
    LocationUpdate_PrePhysics,
    LocationUpdate_PrePhysics_Parallel,
    LocationUpdate_PrePhysics_Post,
    LocationUpdate_PostCloth,
    LocationUpdate_PostCloth_Parallel,
    LocationUpdate_PostCloth_Post,
    LocationUpdate_DebugDraw,
    EventCondition_BonfireNearEnemyCheck,
    HavokWorldUpdate_Pre,
    RenderingSystemUpdate,
    HavokWorldUpdate_Post,
    ChrIns_PreCloth,
    ChrIns_PreClothSafe,
    HavokClothUpdate_Pre_AddRemoveRigidBody,
    HavokClothUpdate_Pre_ClothModelInsSafe,
    HavokClothUpdate_Pre_ClothModelIns,
    HavokClothUpdate_Pre_ClothManager,
    CameraStep,
    DrawParamUpdate,
    GetNPAuthCode,
    SoundStep,
    HavokClothUpdate_Post_ClothManager,
    HavokClothUpdate_Post_ClothModelIns,
    HavokClothVertexUpdateFinishWait,
    ChrIns_PostPhysics,
    ChrIns_PostPhysicsSafe,
    CSDistViewManager_Update,
    HavokAi_SilhouetteGeneratorHelper_Begin,
    WorldChrMan_PostPhysics,
    GameFlowInGame_MoveMap_PostPhysics_0,
    HavokAi_SilhouetteGeneratorHelper_End,
    DmgMan_Pre,
    DmgMan_ShapeCast,
    DmgMan_Post,
    GameFlowInGame_MoveMap_PostPhysics_1_Core0,
    GameFlowInGame_MoveMap_PostPhysics_1_Core1,
    GameFlowInGame_MoveMap_PostPhysics_1_Core2,
    MenuMan,
    WorldChrMan_Update_BackreadRequestPre,
    ChrIns_Update_BackreadRequest,
    WorldChrMan_Update_BackreadRequestPost,
    HavokAi_World,
    WorldAiManager_BeginUpdateFormation,
    WorldAiManager_EndUpdateFormation,
    GameFlowInGame_TestNet,
    GameFlowInGame_InGameMenu,
    GameFlowInGame_TitleMenu,
    GameFlowInGame_CommonMenu,
    GameFlowFrpgNet_Sys,
    GameFlowFrpgNet_Lobby,
    GameFlowFrpgNet_ConnectMan,
    GameFlowFrpgNet_Connect,
    GameFlowStep_Post,
    ScaleformStep,
    FlverResDelayDelectiionEnd,
    Draw_Pre,
    GraphicsStep,
    DebugDrawMemoryBar,
    DbgMenuStep,
    DbgRemoteStep,
    PlaylogSystemStep,
    ReviewMan,
    ReportSystemStep,
    DbgDispStep,
    DrawStep,
    DrawBegin,
    GameSceneDraw,
    AdhocDraw,
    DrawEnd,
    Draw_Post,
    SoundPlayLimitterUpdate,
    BeginShiftWorldPosition,
    FileStep,
    FileStepUpdate_Begin,
    FileStepUpdate_End,
    Flip,
    DelayDeleteStep,
    AiEndCollectGabage,
    RecordHeapStats,
    FrameEnd,
}
