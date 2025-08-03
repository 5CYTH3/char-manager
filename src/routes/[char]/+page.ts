import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/core";
import type { CharData } from "../../schemas/char";

export const load: PageLoad = async ({ params }) => {
  let i: CharData = await invoke("get_char_infos", { name: params.char });
  return i;
};
