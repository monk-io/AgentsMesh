import { getFileService } from "@/lib/wasm-core";

export async function uploadImage(fileOrBlob: File | Blob): Promise<string> {
  const buf = new Uint8Array(await fileOrBlob.arrayBuffer());
  const name = fileOrBlob instanceof File ? fileOrBlob.name : "image.png";
  const type = fileOrBlob.type || "image/png";
  const json = await getFileService().upload_file(buf, name, type);
  const res = JSON.parse(json);
  return res.url ?? res.file_url ?? json;
}
