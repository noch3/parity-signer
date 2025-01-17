//
//  TCAuthorPublicKey.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPublicKey: View {
    var value: AuthorPublicKey
    var body: some View {
        HStack {
            Image(systemName: "circle.fill").foregroundColor(Color("AccentColor")).imageScale(.large)
            VStack (alignment: .leading) {
                Text("Signed with " + value.crypto)
                    .foregroundColor(Color("AccentColor"))
                Text(value.hex)
                    .font(.caption2)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCAuthorPublicKey_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthorPublicKey()
    }
}
*/
