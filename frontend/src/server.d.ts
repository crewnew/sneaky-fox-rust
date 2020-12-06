type DateTime<T> = string
type Local = unknown
type Timestamptz = string
type Regex = string
type ExternalFetcher = string
type InternalFetcher = string
export type Sampler =
	| { type: "RandomSampler"; avg_time: number }
	| { type: "Explicit"; duration: number }
export type EventData =
	| { data_type: "x11_v2"; data: X11EventData }
	| { data_type: "windows_v1"; data: WindowsEventData }
	| { data_type: "app_usage_v2"; data: AppUsageEntry }
	| { data_type: "journald_v1"; data: JournaldEntry }
	| { data_type: "sleep_as_android_v1"; data: SleepAsAndroidEntry }
export type X11EventData = {
	os_info: OsInfo
	desktop_names: string[]
	current_desktop_id: number
	focused_window: number
	ms_since_user_input: number
	ms_until_screensaver: number
	screensaver_window: number
	windows: X11WindowData[]
}
export type X11WindowData = {
	window_id: number
	geometry: X11WindowGeometry
	process: ProcessData | null
	window_properties: { [key: string]: J }
}
export type X11WindowGeometry = {
	x: number
	y: number
	width: number
	height: number
}
export type ProcessData = {
	pid: number
	name: string
	cmd: string[]
	exe: string
	cwd: string
	memory_kB: number
	parent: number | null
	status: string
	start_time: DateTime<Utc>
	cpu_usage: number | null
}
export type OsInfo = {
	os_type: string
	version: string
	batteries: number | null
	hostname: string
	machine_id: string | null
}
export type TagRuleGroup = { global_id: string; data: TagRuleGroupData }
export type TagRuleGroupData = { version: "V1"; data: TagRuleGroupV1 }
export type TagRuleWithMeta = { enabled: boolean; rule: TagRule }
export type TagRule =
	| { type: "TagRegex"; regexes: Regex[]; new_tag: string }
	| { type: "InternalFetcher"; regex: Regex; fetcher: SimpleFetcher }
	| { type: "ExternalFetcher"; regex: Regex; fetcher: ExternalFetcher }
export type TagRuleGroupV1 = {
	name: string
	description: string
	editable: boolean
	rules: TagRuleWithMeta[]
}
