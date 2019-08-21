//
//  ViewController.swift
//  glean-sample-app
//
//  Created by Jan-Erik Rediger on 28.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import UIKit
import Glean

public enum CorePing {
    static let seq = Counter(name: "seq", disabled: false)
}

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view, typically from a nib.

        let g = Glean.shared
        CorePing.seq.add()
    }


}

