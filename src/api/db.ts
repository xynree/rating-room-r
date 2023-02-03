import { invoke } from "@tauri-apps/api"

export async function getTable(tableName: string){
  return invoke(tableName).catch((error) => {
    console.error(error)
    return []
  })
} 

export async function getItem(item_id: number){
  return invoke("get_item", { id: item_id }) as Promise<Item>
}
