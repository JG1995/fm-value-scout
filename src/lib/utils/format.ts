/** Format number with K/M suffixes. */
export function formatAmount(n: number): string {
	if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1).replace(".", ",")}M`;
	if (n >= 1_000) return `${(n / 1_000).toFixed(0)}K`;
	return String(n);
}

/** Format transfer value range with currency symbol. */
export function formatValue(tv: { min: number; max: number }, currency: string): string {
	return `${currency}${formatAmount(tv.min)} - ${currency}${formatAmount(tv.max)}`;
}

/** Format number with thousand separators. */
export function formatFullNumber(n: number): string {
	return String(n).replace(/\B(?=(\d{3})+(?!\d))/g, ".");
}

/** Format wage with currency symbol. */
export function formatWage(w: { weekly_amount: number; unit: string }, currency: string): string {
	return `${currency}${formatFullNumber(w.weekly_amount)} p/w`;
}
