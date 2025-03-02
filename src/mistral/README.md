```zsh
curl --location "https://api.mistral.ai/v1/chat/completions" \
--header 'Content-Type: application/json' \
--header 'Accept: application/json' \
--header "Authorization: Bearer $MISTRAL_API_KEY" \
--data '{
  "model": "ministral-8b-latest",
  "messages": [
    {
      "role": "system",
      "content": "Extract the invoice information to create it. If the date is not specified, use the current date. The list of customers is as follows: John Doe LTD, Foo SAS."
    },
    {
      "role": "user",
      "content": "Create an invoice for John Doe for 3 day of development"
    }
  ],
  "response_format": {
"model": "ministral-8b-latest",
"messages": [
{
"role": "system",
"content": "Extract the invoice information to create it. If the date is not specified, use the current date. The list of customers is as follows: John Doe LTD, Foo SAS."
},
{
"role": "user",
"content": "Create an invoice for John Doe for 3 day of development"
}
],
"response_format": {
"type": "json_schema",
"json_schema": {
"schema": {
"properties": {
"name": {
"title": "Name",
"type": "string"
},
"description": {
"title": "Description",
"type": "string"
},
"date": {
"title": "Date of invoice. Current date if not specified.",
"type": "string"
},
"products": {
"items": {
"properties": {
"description": {
"title": "Description",
"type": "string"
},
"quantity": {
"title": "Quantity",
"description": "Quantity of product in float format. Example: 1.00",
"type": "number"
},
"price": {
"title": "Price",
"description": "Price per unit in float format. Example: 10.00",
"type": "number"
}
},
"additionalProperties": false,
"required": ["description", "quantity", "price"],
"title": "Product",
"type": "object"
},
"title": "Products",
"type": "array"
}
},
"required": ["name", "products"],
"title": "Invoice",
"type": "object",
"additionalProperties": false
},
"name": "invoice",
"strict": true
}
},
"max_tokens": 256,
"temperature": 0
}'
```

```zsh
curl --location "https://api.mistral.ai/v1/chat/completions" \
--header 'Content-Type: application/json' \
--header 'Accept: application/json' \
--header "Authorization: Bearer $MISTRAL_API_KEY" \
--data '{
"model": "ministral-8b-latest",
"messages": [
{
"role": "system",
"content": "Extract the invoice information to create it. If the date is not specified, use the current date. The list of customers is as follows: John Doe LTD, Foo SAS."
},
{
"role": "user",
"content": "Create an invoice for John Doe for 3 day of development"
}
],
"response_format": {
"type": "json_schema",
"json_schema": {
"schema": {
"properties": {
"name": {
"title": "Name",
"type": "string"
},
"description": {
"title": "Description",
"type": "string"
},
"date": {
"title": "Date of invoice. Current date if not specified.",
"type": "string"
},
"products": {
"items": {
"properties": {
"description": {
"title": "Description",
"type": "string"
},
"quantity": {
"title": "Quantity",
"description": "Quantity of product in float format. Example: 1.00",
"type": "number"
},
"price": {
"title": "Price",
"description": "Price per unit in float format. Example: 10.00",
"type": "number"
}
},
"additionalProperties": false,
"required": ["description", "quantity", "price"],
"title": "Product",
"type": "object"
},
"title": "Products",
"type": "array"
}
},
"required": ["name", "products"],
"title": "Invoice",
"type": "object",
"additionalProperties": false
},
"name": "invoice",
"strict": true
}
},
"max_tokens": 256,
"temperature": 0
}'
```