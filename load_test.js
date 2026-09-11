import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  vus: 200, // 50 virtual users
  duration: "30s", // run for 30 seconds
};

export default function () {
  const payload = JSON.stringify({
    jsonrpc: "2.0",
    method: "getAccountInfo",
    params: ["0xabc123"],
    id: 1,
  });

  const params = {
    headers: { "Content-Type": "application/json" },
  };

  const res = http.post("http://127.0.0.1:3000/rpc", payload, params);

  check(res, {
    "status is 200": (r) => r.status === 200,
    "has result": (r) => JSON.parse(r.body).result !== undefined,
  });

  sleep(0.1);
}
