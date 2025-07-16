export enum ElementType {
	Basic = 'Basic',
	Chat = 'Chat',
	Notification = 'Notification',
}

export type StreamLayer = {
	name: string;
	elements: Element[];
};

export type Element = {
	name: string;
	type: ElementType;
};

export type Character = {
	username: string;
	nicknames: string[];
	tts_id: number;
	type: string;
	knowledge: string[];
	isAI: boolean;
	speakText: string;
	init_messages: { role: string; content: string }[];
};

export type Song = {
	id: string;
	title: string;
	author: string;
	link: string;
	duration: number;
	path: string;
	thumbnail_url: string;
	thumbnail_path: string;
	dmca_risk: number;
	approved: boolean;
};

export type Sound = {
	name: string;
	filename: string;
	duration: string;
	source_url: string;
	approved: boolean;
};

export type MidiMapping = {
	note: number;
	control: number;
	name: string;
	type: string;
	action: string;
	sources: any[];
	selection: string;
	inverted: boolean;
};
