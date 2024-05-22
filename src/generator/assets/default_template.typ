// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(title: "", date: (), invoice_day_id: "", enterprise: (), customer: (), lawRules: "", politeness: "", tva: "", body) = {
  // Set the document's basic properties.
  set document(author: enterprise.name, title: title)
  set text(font: "Linux Libertine", lang: "fr", blue.darken(80%))

  // Enterprise information.
  pad(
    top: 0.5em,
    bottom: 0.5em,
    grid(
      columns: (1fr,) * 3,
      gutter: 1em,
      [
        *#enterprise.name* \ \
        #enterprise.email \
        #enterprise.address \
        #enterprise.city #enterprise.postal \
        #enterprise.phone \
        #enterprise.title
      ],
    ),
  )
// Customer information
    pad(
    top: 0.5em,
    bottom: 0.5em,
    grid(
      columns: (2fr, 1fr),
      gutter: 1em,
        pad(
          right: 5em,
          text(0.8em, [Dispensé d'immatriculation au registre du commerce et des sociétés (RCS) et au répertoire des métiers
        
        #rect(fill: blue.lighten(70%),
        inset: 1em,
        grid(
          columns: (8em, 10em),
          gutter: 1em,
          [*Référence :*],
          [#date.display("[year][month][day]")#invoice_day_id],
          [*Date :*],
          date.display("[day]/[month]/[year]")
          ))
        ])
        ),
      [
        *#customer.name* \ \
        #customer.address \
        #customer.city #customer.postal \
      ]),
  )

    // Title row.
    pad(top: 4em, block([#text(weight: 600, 1.2em, [Intitulé :]) #title]))

    body
    
    align(bottom + left, 
    pad( bottom: 2em, politeness))

    align(bottom + left, text(0.9em, gray.darken(60%), lawRules))

  
  align(bottom + center, pad(top: 2em, text(gray.darken(60%), [
    #enterprise.name \
    Siren: #enterprise.siren \
    #if enterprise.tva != "" [N° TVA : #enterprise.tva \ ]
    #enterprise.address \
    #enterprise.city, #enterprise.postal
  ])))
}

#let productsDetails(products, TVANumber) = {
  let isTVAConcerned = TVANumber != ""

  let productsWithTotal = for product in products {
      ([#product.quantity], [#product.description], [#product.price €], [#{product.quantity * product.price} €])
  }
  
  let totalHT = products.fold(0, (acc, product) => {
    acc + product.quantity * product.price
  })
  
  pad(top: 2em,
    table(columns: (auto, 1fr, auto, auto),
    stroke: blue.darken(95%),
      fill: (_, row) => if (row == 0) { blue.darken(10%) } else if calc.even(row) { blue.lighten(90%) } else { white },
    text(white, "Quantité"), text(white, "Désignation"), text(white, "Prix unitaire HT"), text(white, "Prix total HT"),
      ..productsWithTotal
    )
  )

  let TVA_indication = if isTVAConcerned {
    []
  } else {
    [TVA non applicable, art. 293 B du CGI]
  }

  let TVA = if isTVAConcerned {
   ([*TVA*], [#(totalHT * 0.2) €])
   } else {()}

  let Total_TTC = if isTVAConcerned {
   ([*Total TTC*], [#(totalHT * 1.2) €])
   } else {()}
  
  pad(top: 2em,
  align(end + top,
    box(inset: 0.3em, align(start + top, [
      #table(columns: (6em, 6em), align: (left, right),
      stroke: none,
     [*Total HT*], [#totalHT €], ..TVA, ..Total_TTC)
    #text(0.95em, TVA_indication)
    ])
  )
  ))
}