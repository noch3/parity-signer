//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportAddress: View {
    @EnvironmentObject var data: SignerDataModel
    @State var image: UIImage?
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("Scan to export public key")
                    .foregroundColor(Color("textMainColor"))
                    .font(.title)
                    .padding()
                if image != nil {
                    Image(uiImage: image!)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                }
                HStack {
                    NetworkCard(network: data.selectedNetwork)
                    Spacer()
                    Text(data.selectedAddress?.name ?? "none")
                }.padding()
                Button(action: {data.keyManagerModal = .none})
                {
                    Text("Done")
                        .font(.largeTitle)
                        .foregroundColor(Color("AccentColor"))
                        .padding()
                }
            }
            .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
        }
        .onAppear {
            data.lastError = ""
            if data.selectedAddress != nil {
                image = data.exportIdentityQR()
            }
        }
        .gesture(
            DragGesture(minimumDistance: 50)
                .onEnded {amount in
                    print(amount)
                    if amount.translation.width > 0 {
                    data.selectNextAddress()
                    } else {
                        data.selectPreviousAddress()
                    }
                    if data.selectedAddress != nil {
                        image = data.exportIdentityQR()
                    }
                }
        )
    }
}

struct ExportIdentity_Previews: PreviewProvider {
    static var previews: some View {
        ExportAddress()
    }
}
