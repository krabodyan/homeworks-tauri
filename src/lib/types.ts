export type FilterType = "all" | "done" | "todo";

export type Task = {
  id: number;
  done: boolean;
  title: string;
  deadline: Date;
};
