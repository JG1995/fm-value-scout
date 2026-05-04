/** Mirrors backend Rust Player struct — fields the frontend consumes. */
export interface Player {
	unique_id: number;
	name: string;
	nation: string;
	club: string;
	position: string;
	positions: string[];
	age: number;
	height_cm: number;
	transfer_value: TransferValue;
	weekly_wage: Wage;
}

export interface TransferValue {
	min: number;
	max: number;
}

export interface Wage {
	weekly_amount: number;
	unit: string;
}
