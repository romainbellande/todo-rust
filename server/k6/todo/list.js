import http from "k6/http";

export const options = {
  vus: 50,
  duration: '30s'
};

export default function() {
    http.get("https://api.todo.localhost/api/v1/todos");
};
