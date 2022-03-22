import http from "k6/http";

export const options = {
  vus: 50,
  duration: '30s'
};

export default function() {
    http.post("https://api.todo.localhost/api/v1/todos", JSON.stringify({
      name: 'test'
    }), {
      headers: {
        'Content-Type': 'application/json'
      }
    });
};
