import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { AccountService } from '../_services/account.service';

@Component({
  selector: 'app-create-account',
  templateUrl: './create-account.component.html',
  styleUrls: ['./create-account.component.scss'],
})
export class CreateAccountComponent implements OnInit {
  createAccountForm: FormGroup;
  accountTypes = ['Checking', 'Savings'];

  constructor(
    private formBuilder: FormBuilder,
    private accountService: AccountService
  ) {
    this.createAccountForm = this.formBuilder.group({
      accountType: ['', Validators.required],
      accountName: ['', Validators.required],
    });
  }

  ngOnInit(): void {}

  onSubmit() {
    if (this.createAccountForm.valid) {
      const {accountType, accountName} = this.createAccountForm.value;
      this.accountService.createAccount(accountType, accountName).subscribe(
        (response) => {
          console.log(response);
        },
        (error) => {
          console.error(error);
        }
      );

      window.location.reload();
    }
  }
}
