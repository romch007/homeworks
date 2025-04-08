const API_URL = import.meta.env.VITE_API_URL ?? "";

export interface Homework {
  id: number;
  title: String;
  done: boolean;
  subject_id?: number;
  subject?: Subject;
}

export interface Subject {
  id: number;
  name: string;
  created_at: string;
  hex_color?: string;
}

export async function fetcher<Data>(url: string): Promise<Data> {
  return await req("GET", url);
}

export async function req<U>(
  method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
  url: string,
): Promise<U> {
  const resp = await fetch(API_URL + url, {
    method,
  });

  const data = await resp.json();

  return data as U;
}

export async function reqNoResp(
  method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
  url: string,
) {
  await fetch(API_URL + url, {
    method,
  });
}

export async function reqWithBody<T, U>(
  method: "POST" | "PUT",
  url: string,
  body: T,
): Promise<U> {
  const resp = await fetch(API_URL + url, {
    method,
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  const data = await resp.json();

  return data as U;
}

export async function createSubject(
  name: string,
  hexColor?: string,
): Promise<Subject> {
  return await reqWithBody("POST", "/api/subjects", {
    name,
    hex_color: hexColor,
  });
}

export async function updateSubject(
  subjectId: number,
  name?: string,
  hexColor?: string,
) {
  return await reqWithBody("PUT", `/api/subjects/${subjectId}`, {
    name,
    hex_color: hexColor,
  });
}

export async function deleteSubject(subjectId: number) {
  return await reqNoResp("DELETE", `/api/subjects/${subjectId}`);
}
