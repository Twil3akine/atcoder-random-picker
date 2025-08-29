export interface Problem {
	id: string;
	contest_id: string;
	name: string;
	difficulty: number;
};

export type ClosedRange = {
  min: number;
  max: number;
} & { readonly __brand: unique symbol }; 

// for validation
export const createValidRange = (min: number, max: number): ClosedRange | null => {
  return min>max ? null : {min, max, __brand: Symbol('ClosedRange') as never};
}