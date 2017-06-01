# tempo

A Rust templating library, similar to ERB for Ruby.

Example

```
<!DOCTYPE html>
<html>
  <head>
  </head>
  <body>
    <ul>
      <% for i in 0..50 { %>
        <li>Hello <%= i %></li>
      <% } %>
    </ul>
  </body>
</html>
```

