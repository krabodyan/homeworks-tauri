export function parseDate(date: Date): string {
  return date.toISOString().split("T")[0];
}

export function subtractDates(rhs: Date): string {
  const oneDay = 1000 * 60 * 60 * 24;
  const oneWeek = oneDay * 7;
  const now = new Date();
  now.setHours(0);
  const diffInMilliseconds = rhs.getTime() - now.getTime();
  const weeks = Math.round(diffInMilliseconds / oneWeek);
  if (weeks > 0) {
    return `через ${weeks} тиж.`;
  }
  if (weeks < 0) {
    return `${-weeks} тиж. тому`;
  }
  const days = Math.round(diffInMilliseconds / oneDay);
  if (days > 0) {
    return `через ${days} дн.`;
  }
  if (days < 0) {
    return `${-days} дн. тому`;
  }
  return "сьогодні";
}
